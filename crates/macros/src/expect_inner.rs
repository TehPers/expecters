use proc_macro2::TokenStream;
use quote::{ToTokens, quote, quote_spanned};
use syn::{
    AngleBracketedGenericArguments, Expr, Ident, Path, Token, parenthesized,
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    token::Paren,
};

pub struct ExpectInner {
    crate_name: Path,
    _c1: Token![,],
    subject: Expr,
    _c2: Token![,],
    parts: Punctuated<AssertionPart, Token![,]>,
}

impl Parse for ExpectInner {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            crate_name: input.parse()?,
            _c1: input.parse()?,
            subject: input.parse()?,
            _c2: input.parse()?,
            parts: input.call(Punctuated::parse_terminated)?,
        })
    }
}

impl ToTokens for ExpectInner {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        let crate_name = &self.crate_name;
        let subject = &self.subject;
        let parts = &self.parts;

        // Note: it's important to use the input tokens before stringifying
        // them. This is to ensure that the tokens are treated as values instead
        // of arbitrary, meaningless tokens so that LSPs provide real
        // completions for those tokens instead of just letting the user type
        // whatever without any suggestions.
        //
        // As a result, this isn't used until after all the parts are called.
        let frames = parts.iter().map(|part| {
            let name = &part.name;
            quote_spanned!(name.span()=> ::std::stringify!(#name))
        });

        let mut parts = parts.iter();
        let assertion = parts.next_back().map_or_else(
            || quote!(::std::compile_error!("assertion requird")),
            |part| part.to_tokens_assertion(crate_name, frames),
        );
        let modifiers = parts.map(|part| part.to_tokens_modifier(crate_name));

        tokens.extend(quote! {{
            let subject = #crate_name::annotated!(#subject);
            let subject_repr = ::std::string::ToString::to_string(&subject);
            let builder = #crate_name::assertions::AssertionBuilder::__new(subject);
            #(#modifiers)*
            #assertion
        }});
    }
}

/// An assertion-related argument to `expect_inner!(...)`, which matches:
/// - the name (eg. `resource`)
/// - any generics (eg. `::<Foo>`)
/// - any function args, if provided (eg. `(foo, bar)`)
struct AssertionPart {
    name: Ident,
    generics: Option<AngleBracketedGenericArguments>,
    args: Option<AssertionArgs>,
}

impl AssertionPart {
    pub fn to_tokens_modifier(&self, crate_name: &Path) -> TokenStream {
        let mut tokens = TokenStream::new();

        let name = &self.name;
        let generics = &self.generics;
        let args = self
            .args
            .as_ref()
            .map_or_else(|| quote!(()), |args| args.to_tokens(crate_name));
        tokens.extend(quote! {
            let builder = #crate_name::assertions::general::__annotate(
                builder,
                |not_debug| #crate_name::annotated!(not_debug),
            );
            let builder = builder.#name #generics #args;
        });

        tokens
    }

    pub fn to_tokens_assertion(
        &self,
        crate_name: &Path,
        frames: impl IntoIterator<Item = TokenStream>,
    ) -> TokenStream {
        let mut tokens = TokenStream::new();
        let frames = frames.into_iter();

        let name = &self.name;
        let generics = &self.generics;
        let args = self
            .args
            .as_ref()
            .map_or_else(|| quote!(()), |args| args.to_tokens(crate_name));
        tokens.extend(quote! {
            let builder = #crate_name::assertions::general::__annotate(
                builder,
                |not_debug| #crate_name::annotated!(not_debug),
            );
            let assertion = builder.#name #generics #args;
            let cx = #crate_name::assertions::AssertionContext::__new(
                subject_repr,
                #crate_name::source_loc!(),
                {
                    const FRAMES: &[&str] = &[
                        #(#frames,)*
                    ];
                    FRAMES
                },
            );
            #crate_name::assertions::AssertionBuilder::__apply(
                builder,
                cx,
                assertion,
            )
        });

        tokens
    }
}

impl Parse for AssertionPart {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(Self {
            name: input.parse()?,
            generics: input.peek(Token![::]).then(|| input.parse()).transpose()?,
            args: input.peek(Paren).then(|| input.parse()).transpose()?,
        })
    }
}

/// The function args provided to an assertion part, which matches syntax like
/// `(foo, bar)`.
struct AssertionArgs {
    _paren: Paren,
    args: Punctuated<Expr, Token![,]>,
}

impl Parse for AssertionArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let content;
        Ok(Self {
            _paren: parenthesized!(content in input),
            args: content.parse_terminated(Expr::parse, Token![,])?,
        })
    }
}

impl AssertionArgs {
    fn to_tokens(&self, crate_name: &Path) -> TokenStream {
        let mut tokens = TokenStream::new();

        let args = self.args.iter();
        tokens.extend(quote!((#(#crate_name::annotated!(#args),)*)));

        tokens
    }
}
