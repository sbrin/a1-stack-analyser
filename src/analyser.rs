use crate::payload::payload::Payload;
use crate::provider::base;

pub struct AnalyserOptions {
    provider: dyn base::BaseProvider,
}

pub async fn analyser(opts: &AnalyserOptions) -> Payload {
    let provider = &opts.provider;
    let mut pl = Payload::new("main", "/");
    let path = provider.base_path();
    pl.recurse(provider, path).await;

    pl
}
