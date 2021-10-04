use proc_macro::TokenStream;
use syn::{parse_macro_input, Data, DataStruct, DeriveInput, Fields, FieldsNamed, Ident};

#[proc_macro_derive(Builder)]
pub fn derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    // println!("{:#?}", ast);

    let name = &ast.ident;
    let builder_name = format!("{}Builder", name);
    let builder_ident = Ident::new(&builder_name, name.span());
    // match ast.data {
    //     Data::Enum(ref e) => {}
    //     Data::Struct(ref st) => match st.fields {
    //         Fields::Named(ref named) => {}
    //         Fields::Unnamed(ref unnamed) => {}
    //         Fields::Unit => {}
    //     },
    //     Data::Union(ref u) => {}
    // }
    let fields = if let Data::Struct(DataStruct {
        fields: Fields::Named(FieldsNamed { ref named, .. }),
        ..
    }) = &ast.data
    {
        named
    } else {
        unimplemented!()
    };
    println!("name: {}", name);
    println!("builder_name: {}", builder_name);
    // println!("fields: {:#?}", fields);
    let optionized = fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        quote::quote! {
            #ident: std::option::Option<#ty> // should have any comma
        }
    });
    let methods = fields.iter().map(|f| {
        let ident = &f.ident;
        let ty = &f.ty;
        quote::quote! {
            pub fn #ident(&mut self, #ident: #ty) -> &mut Self {
                self.#ident = Some(#ident);
                self
            }
        }
    });
    let build_fields = fields.iter().map(|f| {
        let ident = &f.ident;
        // let message = format!("{:?} is not set", ident);
        quote::quote! {
            #ident: self.#ident.clone().ok_or(concat!(stringify!(#ident), " is not set"))?
        }
    });
    let build_method = quote::quote! {
        pub fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
            let r = #name {
                #(#build_fields,)*
            };
            Ok(r)
        }
    };
    // #(#optionized,)* means iterator
    // fields means PunctuatedFields
    let builder = quote::quote! {
        struct #builder_ident {
            // #fields,
            #(#optionized,)*
            // executable: Option<String>,
            // args: Option<Vec<String>>,
            // env: Option<Vec<String>>,
            // current_dir: Option<String>,
        }

        impl #name {
            fn builder() -> #builder_ident {
                #builder_ident{
                    executable: None,
                    args: None,
                    env: None,
                    current_dir: None,
                }
            }
        }
        impl #builder_ident {
            // fn executable(&mut self, executable: String) -> &mut Self {
            //     self.executable = Some(executable);
            //     self
            // }
            // fn args(&mut self, args: Vec<String>) -> &mut Self {
            //     self.args = Some(args);
            //     self
            // }
            // fn env(&mut self, env: Vec<String>) -> &mut Self {
            //     self.env = Some(env);
            //     self
            // }
            // fn current_dir(&mut self, current_dir: String) -> &mut Self {
            //     self.current_dir = Some(current_dir);
            //     self
            // }
            #(#methods)*

            #build_method

            // fn build(&mut self) -> Result<#name, Box<dyn std::error::Error>> {
            //     let r = #name {
            //         executable: self.executable.clone().ok_or("xxx".to_string())?,
            //         args: self.args.clone().ok_or("xxx".to_string())?,
            //         env: self.env.clone().ok_or("xxx".to_string())?,
            //         current_dir: self.current_dir.clone().ok_or("xxx".to_string())?,
            //     };
            //     Ok(r)
            // }
        }
    };

    builder.into()
}
