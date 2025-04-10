create_cluster:
	kind create cluster --config kind-config.yaml
linkerd_pre_check:
	linkerd check --pre
gateway_api_install:
	kubectl apply -f https://github.com/kubernetes-sigs/gateway-api/releases/download/v1.2.1/standard-install.yaml
linkerd_crds_install:
	linkerd install --crds | kubectl apply -f -
linkerd_install:
	linkerd install | kubectl apply -f -
ingress_install:
	kubectl apply -f https://raw.githubusercontent.com/kubernetes/ingress-nginx/main/deploy/static/provider/cloud/deploy.yaml
linkerd_check:
	linkerd check
linkerd_viz_install:
	linkerd viz install | kubectl apply -f -
injectall:
	kubectl annotate namespace demo linkerd.io/inject=enabled
uninjectall:
	kubectl annotate namespace demo linkerd.io/inject- --overwrite
inject:
	kubectl get deploy -n demo -o yaml | linkerd inject - | kubectl apply -f -
uninject:
	kubectl get deploy -n demo -o yaml | linkerd uninject - | kubectl apply -f -
