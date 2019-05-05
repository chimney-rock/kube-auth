pub mod authentication;

/// A trait applied to all Kubernetes resources.
pub trait Resource {
  /// The API version of the resource. This is a composite of [`Resource::group`] and [`Resource::version`] (eg `"apiextensions.k8s.io/v1beta1"`)
  /// or just the version for resources without a group (eg `"v1"`).
  fn api_version() -> &'static str where Self: Sized;

  /// The kind of the resource.
  fn kind() -> &'static str where Self: Sized;

  /// The version of the resource.
  fn version() -> &'static str where Self: Sized;
}
