#!/bin/bash
# =============================================================================
# RABBITMQ CLUSTER INSTALLATION SCRIPT
# =============================================================================
# Este script instala RabbitMQ no cluster Kubernetes
# Execução: DA MÁQUINA LOCAL (com kubectl configurado)
# Pré-requisitos: Cluster K3s funcionando
# =============================================================================

echo "🐰 Instalando RabbitMQ no cluster..."

# Instalar o RabbitMQ Cluster Operator oficial
echo "📦 Instalando RabbitMQ Cluster Operator..."
kubectl apply -f "https://github.com/rabbitmq/cluster-operator/releases/latest/download/cluster-operator.yml"

# Instalar Local Path Provisioner para storage persistente
echo "💾 Configurando storage persistente..."
kubectl apply -f https://raw.githubusercontent.com/rancher/local-path-provisioner/master/deploy/local-path-storage.yaml
kubectl annotate storageclass local-path storageclass.kubernetes.io/is-default-class=true

# Aguardar operator ficar pronto
echo "⏳ Aguardando operator ficar pronto..."
kubectl wait --for=condition=available --timeout=300s deployment/rabbitmq-cluster-operator -n rabbitmq-system

# Aplicar configuração customizada do RabbitMQ
echo "⚙️ Criando cluster RabbitMQ..."
kubectl apply -f rabbitmq.yml

echo "✅ RabbitMQ instalação iniciada!"
echo ""
echo "📋 Para obter credenciais:"
echo "   username=\$(kubectl get secret friday-rabbitmq-default-user -o jsonpath='{.data.username}' | base64 --decode)"
echo "   password=\$(kubectl get secret friday-rabbitmq-default-user -o jsonpath='{.data.password}' | base64 --decode)"
echo ""
echo "🌐 Acesso via subdomínio:"
echo "   https://rabbitmq.z33p.com"
echo ""
echo "🔗 Port-forward local:"
echo "   kubectl port-forward service/friday-rabbitmq 15672"

# kubectl port-forward "service/friday-rabbitmq" 15672