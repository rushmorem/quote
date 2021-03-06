#![cfg_attr(feature = "cargo-clippy", allow(blacklisted_name))]

use std::borrow::Cow;

extern crate proc_macro2;
#[macro_use]
extern crate quote;

use proc_macro2::{Ident, Span, TokenStream};
use quote::TokenStreamExt;

struct X;

impl quote::ToTokens for X {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.append(Ident::new("X", Span::call_site()));
    }
}

#[test]
fn test_quote_impl() {
    let tokens = quote! {
        impl<'a, T: ToTokens> ToTokens for &'a T {
            fn to_tokens(&self, tokens: &mut TokenStream) {
                (**self).to_tokens(tokens)
            }
        }
    };

    let expected = concat!(
        "impl < 'a , T : ToTokens > ToTokens for & 'a T { ",
        "fn to_tokens ( & self , tokens : & mut TokenStream ) { ",
        "( * * self ) . to_tokens ( tokens ) ",
        "} ",
        "}"
    );

    assert_eq!(expected, tokens.to_string());
}

#[test]
fn test_substitution() {
    let x = X;
    let tokens = quote!(#x <#x> (#x) [#x] {#x});

    let expected = "X < X > ( X ) [ X ] { X }";

    assert_eq!(expected, tokens.to_string());
}

#[test]
fn test_iter() {
    let primes = &[X, X, X, X];

    assert_eq!("X X X X", quote!(#(#primes)*).to_string());

    assert_eq!("X , X , X , X ,", quote!(#(#primes,)*).to_string());

    assert_eq!("X , X , X , X", quote!(#(#primes),*).to_string());
}

#[test]
fn test_advanced() {
    let generics = quote!( <'a, T> );

    let where_clause = quote!( where T: Serialize );

    let field_ty = quote!(String);

    let item_ty = quote!(Cow<'a, str>);

    let path = quote!(SomeTrait::serialize_with);

    let value = quote!(self.x);

    let tokens = quote! {
        struct SerializeWith #generics #where_clause {
            value: &'a #field_ty,
            phantom: ::std::marker::PhantomData<#item_ty>,
        }

        impl #generics ::serde::Serialize for SerializeWith #generics #where_clause {
            fn serialize<S>(&self, s: &mut S) -> Result<(), S::Error>
                where S: ::serde::Serializer
            {
                #path(self.value, s)
            }
        }

        SerializeWith {
            value: #value,
            phantom: ::std::marker::PhantomData::<#item_ty>,
        }
    };

    let expected = concat!(
        "struct SerializeWith < 'a , T > where T : Serialize { ",
        "value : & 'a String , ",
        "phantom : :: std :: marker :: PhantomData < Cow < 'a , str > > , ",
        "} ",
        "impl < 'a , T > :: serde :: Serialize for SerializeWith < 'a , T > where T : Serialize { ",
        "fn serialize < S > ( & self , s : & mut S ) -> Result < ( ) , S :: Error > ",
        "where S : :: serde :: Serializer ",
        "{ ",
        "SomeTrait :: serialize_with ( self . value , s ) ",
        "} ",
        "} ",
        "SerializeWith { ",
        "value : self . x , ",
        "phantom : :: std :: marker :: PhantomData :: < Cow < 'a , str > > , ",
        "}"
    );

    assert_eq!(expected, tokens.to_string());
}

#[test]
fn test_integer() {
    let ii8 = -1i8;
    let ii16 = -1i16;
    let ii32 = -1i32;
    let ii64 = -1i64;
    let iisize = -1isize;
    let uu8 = 1u8;
    let uu16 = 1u16;
    let uu32 = 1u32;
    let uu64 = 1u64;
    let uusize = 1usize;

    let tokens = quote! {
        #ii8 #ii16 #ii32 #ii64 #iisize
        #uu8 #uu16 #uu32 #uu64 #uusize
    };
    let expected = "-1i8 -1i16 -1i32 -1i64 -1isize 1u8 1u16 1u32 1u64 1usize";
    assert_eq!(expected, tokens.to_string());
}

#[test]
fn test_floating() {
    let e32 = 2.345f32;

    let e64 = 2.345f64;

    let tokens = quote! {
        #e32
        #e64
    };
    let expected = concat!("2.345f32 2.345f64");
    assert_eq!(expected, tokens.to_string());
}

#[test]
fn test_char() {
    let zero = '\0';
    let pound = '#';
    let quote = '"';
    let apost = '\'';
    let newline = '\n';
    let heart = '\u{2764}';

    let tokens = quote! {
        #zero #pound #quote #apost #newline #heart
    };
    let expected = "'\\u{0}' '#' '\\\"' '\\'' '\\n' '\\u{2764}'";
    assert_eq!(expected, tokens.to_string());
}

#[test]
fn test_str() {
    let s = "\0 a 'b \" c";
    let tokens = quote!(#s);
    let expected = "\"\\u{0} a \\'b \\\" c\"";
    assert_eq!(expected, tokens.to_string());
}

#[test]
fn test_string() {
    let s = "\0 a 'b \" c".to_string();
    let tokens = quote!(#s);
    let expected = "\"\\u{0} a \\'b \\\" c\"";
    assert_eq!(expected, tokens.to_string());
}

#[test]
fn test_ident() {
    let foo = Ident::new("Foo", Span::call_site());
    let bar = Ident::new(&format!("Bar{}", 7), Span::call_site());
    let tokens = quote!(struct #foo; enum #bar {});
    let expected = "struct Foo ; enum Bar7 { }";
    assert_eq!(expected, tokens.to_string());
}

#[test]
fn test_duplicate() {
    let ch = 'x';

    let tokens = quote!(#ch #ch);

    let expected = "'x' 'x'";
    assert_eq!(expected, tokens.to_string());
}

#[test]
fn test_fancy_repetition() {
    let foo = vec!["a", "b"];
    let bar = vec![true, false];

    let tokens = quote! {
        #(#foo: #bar),*
    };

    let expected = r#""a" : true , "b" : false"#;
    assert_eq!(expected, tokens.to_string());
}

#[test]
fn test_nested_fancy_repetition() {
    let nested = vec![vec!['a', 'b', 'c'], vec!['x', 'y', 'z']];

    let tokens = quote! {
        #(
            #(#nested)*
        ),*
    };

    let expected = "'a' 'b' 'c' , 'x' 'y' 'z'";
    assert_eq!(expected, tokens.to_string());
}

#[test]
fn test_empty_repetition() {
    let tokens = quote!(#(a b)* #(c d),*);
    assert_eq!("", tokens.to_string());
}

#[test]
fn test_variable_name_conflict() {
    // The implementation of `#(...),*` uses the variable `_i` but it should be
    // fine, if a little confusing when debugging.
    let _i = vec!['a', 'b'];
    let tokens = quote! { #(#_i),* };
    let expected = "'a' , 'b'";
    assert_eq!(expected, tokens.to_string());
}

#[test]
fn test_empty_quote() {
    let tokens = quote!();
    assert_eq!("", tokens.to_string());
}

#[test]
fn test_box_str() {
    let b = "str".to_owned().into_boxed_str();
    let tokens = quote! { #b };
    assert_eq!("\"str\"", tokens.to_string());
}

#[test]
fn test_cow() {
    let owned: Cow<Ident> = Cow::Owned(Ident::new("owned", Span::call_site()));

    let ident = Ident::new("borrowed", Span::call_site());
    let borrowed = Cow::Borrowed(&ident);

    let tokens = quote! { #owned #borrowed };
    assert_eq!("owned borrowed", tokens.to_string());
}

#[test]
fn test_closure() {
    fn field_i(i: usize) -> Ident {
        Ident::new(&format!("__field{}", i), Span::call_site())
    }

    let fields = (0usize..3)
        .map(field_i as fn(_) -> _)
        .map(|var| quote! { #var });

    let tokens = quote! { #(#fields)* };
    assert_eq!("__field0 __field1 __field2", tokens.to_string());
}

#[test]
fn test_append_tokens() {
    let mut a = quote!(a);
    let b = quote!(b);
    a.append_all(b);
    assert_eq!("a b", a.to_string());
}
