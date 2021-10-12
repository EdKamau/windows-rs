use super::*;

pub enum Gen {
    Absolute,
    Relative(&'static str),
    Crate((&'static str, Vec<&'static str>, std::collections::BTreeSet<&'static str>))
}

impl Gen {
    pub fn namespace(&self, namespace: &str) -> TokenStream {
        match self {
            Self::Absolute => {
                let mut tokens = TokenStream::with_capacity();

                for namespace in namespace.split('.') {
                    tokens.push_str(namespace);
                    tokens.push_str("::");
                }

                tokens
            }
            Self::Relative(relative) => gen_relative(namespace, relative),
            Self::Crate((relative, includes, crates)) => {
                for include in includes {
                    if contains_namespace(namespace, include) {
                        return gen_relative(namespace, relative);
                    }
                }

                for dependency in crates {
                    if contains_namespace(dependency, namespace) {
                        let mut result = format!("::{}::", dependency.replace('.', "_").to_lowercase());
                        if namespace.len() > dependency.len() {
                            result.push_str(&namespace[dependency.len() + 1..].replace('.', "::"));
                            result.push_str("::");
                        }
                        return result.into();
                    }
                }

                panic!("Unexpected namespace relative:{} namespace:{}", relative, namespace);
            }
        }
    }
}

fn gen_relative(namespace: &str, relative: &str) -> TokenStream {
    if namespace == relative {
        return TokenStream::new();
    }

    let mut relative = relative.split('.').peekable();
    let mut namespace = namespace.split('.').peekable();

    while relative.peek() == namespace.peek() {
        if relative.next().is_none() {
            break;
        }

        namespace.next();
    }

    let mut tokens = TokenStream::with_capacity();

    for _ in 0..relative.count() {
        tokens.push_str("super::");
    }

    for namespace in namespace {
        tokens.push_str(namespace);
        tokens.push_str("::");
    }

    tokens
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_namespace() {
        assert_eq!(
            Gen::Absolute.namespace("Windows.Foundation").as_str(),
            "Windows::Foundation::"
        );

        assert_eq!(
            Gen::Relative("Windows")
                .namespace("Windows.Foundation")
                .as_str(),
            "Foundation::"
        );

        assert_eq!(
            Gen::Relative("Windows.Foundation")
                .namespace("Windows.Foundation")
                .as_str(),
            ""
        );

        assert_eq!(
            Gen::Relative("Windows.Foundation.Collections")
                .namespace("Windows.Foundation")
                .as_str(),
            "super::"
        );
    }
}
