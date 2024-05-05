# Instalação do Fleet
helm repo add fleet https://rancher.github.io/fleet-helm-charts/

helm -n cattle-fleet-system install --create-namespace --wait fleet-crd \
    fleet/fleet-crd
helm -n cattle-fleet-system install --create-namespace --wait fleet \
    fleet/fleet