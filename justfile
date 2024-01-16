set dotenv-load

default: (cli "--help")

cli command:
 @cargo run -- {{command}}

apply-crd: (cli "crd-gen | kubectl apply --context='k3d-spacetraders' -f -")

init-cluster: create-cluster apply-crd

create-cluster:
 @k3d cluster create --config k8s/k3d.yaml

delete-cluster:
 @k3d cluster delete --config k8s/k3d.yaml

migrate:
 @DATABASE_URL=$DATABASE_URL cargo run -p migration -- up
