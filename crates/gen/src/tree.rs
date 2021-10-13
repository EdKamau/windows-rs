use super::*;

// TODO: move crate-specific generation in the cates project?

// TODO: use "module" to shorten feature names and collapse namespaces down to be relative to module/crate.
pub fn gen_crate_source_tree(module:&str, includes: &Vec<&'static str>, crates: &std::collections::BTreeSet<&'static str>) -> TokenStream {
    let reader = TypeReader::get();
    gen_crate_namespaces(module, includes, &reader.types.namespaces, crates)
}

fn gen_crate_namespaces(module:&str, includes: &Vec<&'static str>, namespaces: & BTreeMap<&'static str, TypeTree>, crates: &std::collections::BTreeSet<&'static str>) -> TokenStream {
    let mut tokens = TokenStream::with_capacity();

    for (name, tree) in namespaces {
        if module == "Windows.UI" && tree.namespace.starts_with("Windows.UI.Xaml") {
            continue;
        }

        if tree.namespace == module || tree.namespace.starts_with(&format!("{}.", module)) {
            let gen = Gen::Crate((tree.namespace, includes.clone(), crates.clone())); // TODO: clone?!
            let name = to_ident(name);
            let nested = gen_crate_namespaces(module, includes, &tree.namespaces, crates);
            let types =     tree.types
            .iter()
            .map(move |t| gen_type_entry(t.1, &gen));

            if tree.namespace.len() > module.len() {
                tokens.combine(&quote! {
                    pub mod #name {
                        #nested
                        #(#types)*
                    }
                });
            } else {
                tokens.combine(&quote! {
                    #nested
                    #(#types)*
                });
            }
         } else {
            tokens.combine(&gen_crate_namespaces(module, includes, &tree.namespaces, crates));
         }
    }

    tokens
}

pub fn contains_namespace(module: &str, namespace: &str) -> bool {
    if module == "Windows.UI" && namespace.starts_with("Windows.UI.Xaml") {
        return false;
    }

    namespace == module || namespace.starts_with(&format!("{}.", module))
}

pub fn gen_source_tree() -> TokenStream {
    let reader = TypeReader::get();

    namespace_iter(&reader.types).fold(TokenStream::new(), |mut accum, n| {
        accum.combine(&n);
        accum
    })
}

pub fn namespace_iter(tree: &TypeTree) -> impl Iterator<Item = TokenStream> + '_ {
    let gen = Gen::Relative(tree.namespace);

    tree.types
        .iter()
        .map(move |t| gen_type_entry(t.1, &gen))
        .chain(gen_namespaces(&tree.namespaces))
}

fn gen_namespaces<'a>(
    namespaces: &'a BTreeMap<&'static str, TypeTree>,
) -> impl Iterator<Item = TokenStream> + 'a {
    namespaces.iter().map(move |(name, tree)| {
        if tree.include {
            let name = to_ident(name);
            let tokens = namespace_iter(tree);

            quote! {
                // TODO: https://github.com/microsoft/windows-rs/issues/212
                // TODO: https://github.com/microsoft/win32metadata/issues/380
                #[allow(unused_variables, non_upper_case_globals, non_snake_case, unused_unsafe, non_camel_case_types, dead_code, clippy::all)]
                pub mod #name {
                    #(#tokens)*
                }
            }
         } else {
             TokenStream::new()
         }
    })
}

fn namespace_feature(namespace: &str) -> String {
    namespace.replace('.', "_").into()
}

fn gen_type_entry(entry: &TypeEntry, gen: &Gen) -> TokenStream {
    if entry.include == TypeInclude::None {
        return TokenStream::new();
    }

    let mut tokens = TokenStream::new();

    // TODO: replace with regular for loop once multi-arch struct work is complete.
    for def in entry.def.first().iter() {
        // for def in &entry.def {
        tokens.combine(&match def {
            ElementType::TypeDef(def) => gen_type(&def.clone().with_generics(), gen, entry.include),
            ElementType::MethodDef(def) => gen_function(def, gen),
            ElementType::Field(def) => gen_constant(def, gen),
            _ => unimplemented!(),
        });
    }

    tokens
}

fn gen_type(def: &TypeDef, gen: &Gen, include: TypeInclude) -> TokenStream {
    match def.kind() {
        TypeKind::Interface => {
            if def.is_winrt() {
                gen_interface(&def.clone().with_generics(), gen, include)
            } else {
                gen_com_interface(def, gen, include)
            }
        }
        TypeKind::Class => Class(def.clone().with_generics()).gen(gen, include),
        TypeKind::Enum => gen_enum(def, gen, include),
        TypeKind::Struct => gen_struct(def, gen),
        TypeKind::Delegate => {
            if def.is_winrt() {
                gen_delegate(def, gen)
            } else {
                gen_callback(def, gen)
            }
        }
    }
}
