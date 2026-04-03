#!/bin/bash
# =============================================================================
# POSTGRESQL INSTALLATION SCRIPT
# =============================================================================
# Este script instala PostgreSQL no cluster Kubernetes usando Helm
# e executa os scripts de inicialização do banco de dados.
#
# Execução: DA MÁQUINA LOCAL (com kubectl configurado)
# Pré-requisitos: Cluster K3s funcionando + Helm instalado + psql instalado
# =============================================================================

set -e

echo "🐘 Instalando PostgreSQL no cluster..."

# Adicionar repositório oficial da Bitnami ao Helm
echo "📦 Adicionando repositório Bitnami..."
helm repo add bitnami https://charts.bitnami.com/bitnami

# Atualizar repositórios do Helm para versões mais recentes
echo "🔄 Atualizando repositórios Helm..."
helm repo update

# Instalar PostgreSQL usando configurações personalizadas
echo "⚙️ Instalando PostgreSQL com configurações customizadas..."
helm install friday-postgres bitnami/postgresql -f postgres-values.yaml --namespace default

# Aguardar o pod ficar pronto
echo "⏳ Aguardando PostgreSQL ficar pronto..."
kubectl rollout status statefulset/friday-postgres-postgresql --namespace default --timeout=180s

echo "✅ PostgreSQL instalado com sucesso!"
echo ""
echo "📋 Informações de acesso:"
echo "   - Interno: friday-postgres-postgresql.default.svc.cluster.local:5432"
echo "   - Port-forward: kubectl port-forward svc/friday-postgres-postgresql 5432:5432"
echo "   - Connection string: postgresql://postgres:<senha>@friday-postgres-postgresql.default.svc.cluster.local:5432/postgres"
echo ""

echo ""
echo "✅ Banco de dados inicializado com sucesso!"
echo ""
echo "🔑 Para obter a senha:"
echo "   kubectl get secret friday-postgres-postgresql -n default -o jsonpath='{.data.password}' | base64 -d"
