use proc_macro::TokenStream;
use quote::quote;

/// This procedural macro will introduce a mutable
/// [`PoolConnection`][`sqlx::pool::PoolConnection`] instance called
/// `connection`. So you can just operate on that. Such as making queries.
#[proc_macro]
pub fn get_connection(_item: TokenStream) -> TokenStream {
    quote! {
      use crate::DB_POOL;
      let pool_guard = DB_POOL.lock().await;
      let mut connection = pool_guard.as_ref().unwrap().acquire().await.unwrap();
      drop(pool_guard);
    }
    .into()
}
