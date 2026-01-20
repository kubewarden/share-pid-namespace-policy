[!IMPORTANT]
**Notice:**
Starting from Kubewarden release 1.32.0, all code from this repository has been merged into [github.com/kubewarden/policies](https://github.com/kubewarden/policies), which is now a monorepo containing policies.
Please refer to that repository for future updates and development.
**This repository is now archived. Development continues in the new location.**


[![Kubewarden Policy Repository](https://github.com/kubewarden/community/blob/main/badges/kubewarden-policies.svg)](https://github.com/kubewarden/community/blob/main/REPOSITORIES.md#policy-scope)
[![Stable](https://img.shields.io/badge/status-stable-brightgreen?style=for-the-badge)](https://github.com/kubewarden/community/blob/main/REPOSITORIES.md#stable)

# share-pid-namespace-policy

This policy will reject pods that have pod configured to share process namespace.
In other words, this policy rejects Pod that have `spec.shareProcessNamespace` set to `true`.

## Settings

This policy has no configurable settings.

## Example

Considering the policy applied with the following definition:

```yaml
apiVersion: policies.kubewarden.io/v1
kind: ClusterAdmissionPolicy
metadata:
  annotations:
    io.kubewarden.policy.category: Resource validation
    io.kubewarden.policy.severity: medium
  name: share-pid-namespace-policy
spec:
  module: ghcr.io/kubewarden/policies/share-pid-namespace-policy:v0.1.0
  settings: {}
  rules:
    - apiGroups: [""]
      apiVersions: ["v1"]
      resources: ["pods"]
      operations: ["CREATE"]
    - apiGroups: [""]
      apiVersions: ["v1"]
      resources: ["replicationcontrollers"]
      operations: ["CREATE", "UPDATE"]
    - apiGroups: ["apps"]
      apiVersions: ["v1"]
      resources: ["deployments", "replicasets", "statefulsets", "daemonsets"]
      operations: ["CREATE", "UPDATE"]
    - apiGroups: ["batch"]
      apiVersions: ["v1"]
      resources: ["jobs", "cronjobs"]
      operations: ["CREATE", "UPDATE"]
  mutating: false
```

Pods with the `shareProcessNamespace` will be rejected:

```yaml
apiVersion: v1
kind: Pod
metadata:
  name: nginx
spec:
  shareProcessNamespace: true
  containers:
    - name: nginx
      image: nginx:1.14.2
      ports:
        - containerPort: 80
```
