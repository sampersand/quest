use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput, Meta, NestedMeta, MetaList, MetaNameValue, Lit};

#[proc_macro_derive(Named, attributes(quest))]
pub fn derive_named_type(input: TokenStream) -> TokenStream {
	let DeriveInput { ident, attrs, .. } = parse_macro_input!(input);

	let mut typename = ident.to_string();
	let mut cratename = quote!(::qvm::value::NamedType);

	for attr in attrs {
		if !attr.path.is_ident("quest") {
			continue;
		}

		if let Ok(Meta::List(list)) = attr.parse_meta() {
			for ele in list.nested {
				if let NestedMeta::Meta(Meta::NameValue(MetaNameValue { ref path, lit: Lit::Str(ref name), .. })) = ele {
					if path.is_ident("typename") {
						typename = name.value();
					} else if path.is_ident("crate_name") {
						cratename = format!("{}::value::NamedType", name.value()).parse().expect("bad cratename");
					}
				}
			}
		}
	}

	let output = quote! {
		impl #cratename for #ident {
			const TYPENAME: &'static str = #typename;
		}
	};

	output.into()
}


#[proc_macro_derive(QuestType, attributes(quest))]
pub fn derive_quest_type(input: TokenStream) -> TokenStream {
   let DeriveInput { ident, attrs, .. } = parse_macro_input!(input);

   let mut typename = ident.to_string();
   let mut cratename = "::qvm".to_string();
   let mut toskip: Vec<String> = vec![];

   for attr in attrs {
   	if !attr.path.is_ident("quest") {
   		continue;
   	}

		let list =
			if let Ok(Meta::List(list)) = attr.parse_meta() {
				list
			} else {
				panic!("unsupported syntax given");
			};

		for ele in list.nested {
			let nested = 
				if let NestedMeta::Meta(nested) = ele { 
					nested
				} else {
					panic!("unsupported syntax given");
				};

			match nested {
				Meta::NameValue(MetaNameValue { path, lit: Lit::Str(name), .. }) if path.is_ident("typename")
					=> typename = name.value(),
				Meta::NameValue(MetaNameValue { path, lit: Lit::Str(name), .. }) if path.is_ident("crate_name")
					=> cratename = name.value(),
				Meta::List(MetaList { path, nested, .. }) if path.is_ident("skip")
					=> toskip = nested.into_iter().map(|traitname| 
						if let NestedMeta::Lit(Lit::Str(name)) = traitname {
							name.value()
						} else {
							panic!("unknown traitname")
						}
					).collect(),
				_ => panic!("unknown ele")
			}
		}
	}

	let named = 
		if toskip.iter().any(|x| x.as_str() == "NamedType") {
			quote!()
		} else {
			let traitname: proc_macro2::TokenStream = format!("{}::value::NamedType", cratename).parse().expect("bad crate_name");

			quote! {
				impl #traitname for #ident {
					const TYPENAME: &'static str = #typename;
				}
			}
		};

	let shallow = 
		if toskip.iter().any(|x| x.as_str() == "ShallowClone") {
			quote!()
		} else {
			let traitname: proc_macro2::TokenStream = format!("{}::ShallowClone", cratename).parse().expect("bad crate_name");
			let result: proc_macro2::TokenStream = format!("{}::Result<Self>", cratename).parse().expect("bad crate_name");
			quote! {
				impl #traitname for #ident {
					// note that this only works for clonable types
					fn shallow_clone(&self) -> #result { self.clone() }
				}
			}
		};

	let deep = 
		if toskip.iter().any(|x| x.as_str() == "DeepClone") {
			quote!()
		} else {
			let traitname: proc_macro2::TokenStream = format!("{}::DeepClone", cratename).parse().expect("bad crate_name");
			let result: proc_macro2::TokenStream = format!("{}::Result<Self>", cratename).parse().expect("bad crate_name");
			quote! {
				impl #traitname for #ident {
					// note that this only works for clonable types
					fn deep_clone(&self) -> #result { self.clone() }
				}
			}
		};

	let externtrait: proc_macro2::TokenStream = format!("{}::value::ExternType", cratename).parse().expect("bad crate_name");

	quote!( #named #shallow #deep impl #externtrait for #ident {}).into()
}

// use proc_macro::{self, TokenStream};
// use quote::quote;
// use syn::{parse_macro_input, DataEnum, DataUnion, DeriveInput, FieldsNamed, FieldsUnnamed};

// #[proc_macro_derive(Describe)]
// pub fn describe(input: TokenStream) -> TokenStream {
//     let DeriveInput { ident, data, .. } = parse_macro_input!(input);

//     let description = match data {
//     syn::Data::Struct(s) => match s.fields {
//         syn::Fields::Named(FieldsNamed { named, .. }) => {
//         let idents = named.iter().map(|f| &f.ident);
//         format!(
//             "a struct with these named fields: {}",
//             quote! {#(#idents), *}
//         )
//         }
//         syn::Fields::Unnamed(FieldsUnnamed { unnamed, .. }) => {
//         let num_fields = unnamed.iter().count();
//         format!("a struct with {} unnamed fields", num_fields)
//         }
//         syn::Fields::Unit => format!("a unit struct"),
//     },
//     syn::Data::Enum(DataEnum { variants, .. }) => {
//         let vs = variants.iter().map(|v| &v.ident);
//         format!("an enum with these variants: {}", quote! {#(#vs),*})
//     }
//     syn::Data::Union(DataUnion {
//         fields: FieldsNamed { named, .. },
//         ..
//     }) => {
//         let idents = named.iter().map(|f| &f.ident);
//         format!("a union with these named fields: {}", quote! {#(#idents),*})
//     }
//     };

//     let output = quote! {
//     impl #ident {
//         fn describe() {
//         println!("{} is {}.", stringify!(#ident), #description);
//         }
//     }
//     };

//     output.into()
// }
