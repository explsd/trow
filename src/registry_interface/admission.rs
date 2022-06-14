use k8s_openapi::api::core::v1::Pod;
use kube::core::admission::{AdmissionRequest, AdmissionResponse};

#[rocket::async_trait]
pub trait AdmissionValidation {
    // This function signature is very tied to the implementation.
    // If you develop a new front-end and have problems here, we should change it.
    async fn validate_admission(
        &self,
        admission_req: &AdmissionRequest<Pod>,
        host_names: &[String],
    ) -> AdmissionResponse;
}
