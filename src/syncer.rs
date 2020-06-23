pub trait Pusher : Sync {
    type PushResult;
    fn push(&self) -> Self::PushResult;
}

pub trait Fetcher : Sync {
    type FetchResult;
    type MergeResult;

    fn fetch(&self) -> Self::FetchResult;
    fn merge(&self) -> Self::MergeResult;
}

mod git;
