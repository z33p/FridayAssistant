#!/bin/bash
# =============================================================================
# REDIS CLUSTER INSTALLATION SCRIPT
# =============================================================================
# Este script instala Redis no cluster Kubernetes usando Helm
# Execução: DA MÁQUINA LOCAL (com kubectl configurado)
# Pré-requisitos: Cluster K3s funcionando + Helm instalado
# =============================================================================

echo "🚀 Instalando Redis no cluster..."

# Adicionar repositório oficial da Bitnami ao Helm
echo "📦 Adicionando repositório Bitnami..."
helm repo add bitnami https://charts.bitnami.com/bitnami

# Atualizar repositórios do Helm para versões mais recentes
echo "🔄 Atualizando repositórios Helm..."
helm repo update

# Instalar Redis usando configurações personalizadas
echo "⚙️ Instalando Redis com configurações customizadas..."
helm install friday-redis bitnami/redis -f friday-redis.yml

echo "✅ Redis instalado com sucesso!"
echo ""
echo "📋 Informações de acesso:"
echo "   - Interno: friday-redis-master.default.svc.cluster.local:6379"
echo "   - Port-forward: kubectl port-forward svc/friday-redis-master 6379:6379"
echo ""
echo "🧪 Para testar:"
echo "   kubectl run redis-client --restart='Never' --image docker.io/bitnami/redis:8.2.1-debian-12-r0 --command -- sleep infinity"
echo "   kubectl exec -it redis-client -- redis-cli -h friday-redis-master ping"

# Redis can be accessed on the following DNS names from within your cluster:

# friday-redis-master.default.svc.cluster.local for read/write operations (port 6379)
# friday-redis-replicas.default.svc.cluster.local for read-only operations (port 6379)
# To get your password run:

# export REDIS_PASSWORD=$(kubectl get secret --namespace default friday-redis -o jsonpath="{.data.redis-password}" | base64 -d)

# To connect to your Redis; server:

# 1. Run a Redis; pod that you can use as a client:

#    kubectl run --namespace default redis-client --restart='Never'  --env REDIS_PASSWORD=$REDIS_PASSWORD  --image docker.io/bitnami/redis:7.2.4-debian-12-r12 --command -- sleep infinity

#    Use the following command to attach to the pod:

#    kubectl exec --tty -i redis-client \
#    --namespace default -- bash

# 2. Connect using the Redis; CLI:
#    REDISCLI_AUTH="$REDIS_PASSWORD" redis-cli -h friday-redis-master
#    REDISCLI_AUTH="$REDIS_PASSWORD" redis-cli -h friday-redis-replicas

# To connect to your database from outside the cluster execute the following commands:

    # kubectl port-forward --namespace default svc/friday-redis-master 6379:6379
        # & REDISCLI_AUTH="$REDIS_PASSWORD" redis-cli -h 127.0.0.1 -p 6379

# WARNING: There are "resources" sections in the chart not set. Using "resourcesPreset" is not recommended for production. For production installations, please set the following values according to your workload needs:
#   - master.resources
#   - replica.resources
# +info https://kubernetes.io/docs/concepts/configuration/manage-resources-containers/