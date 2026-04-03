#!/bin/bash
# =============================================================================
# KUBE-SYSTEM RIGHTSIZING PATCH SCRIPT
# =============================================================================
# Aplica patches de resources nos componentes do kube-system e rabbitmq-system
# que não possuem arquivos de configuração no repositório.
#
# Uso real observado (kubectl top pods --all-namespaces):
#   coredns:                  ~25Mi / 2m CPU  (req: 100m/70Mi)
#   metrics-server:           ~29Mi / 5m CPU  (req: 100m/70Mi)
#   traefik:                  ~37Mi / 1m CPU  (sem requests/limits)
#   rabbitmq-cluster-operator: ~22Mi / 2m CPU (req: 200m/500Mi)
#
# Execução: DA MÁQUINA LOCAL (com kubectl configurado)
# =============================================================================

set -e

echo "⚙️ Aplicando rightsizing nos componentes do kube-system e rabbitmq-system..."

# -----------------------------------------------------------------------------
# CoreDNS — uso real ~25Mi, tinha req=100m/70Mi
# -----------------------------------------------------------------------------
echo "🔧 Patching coredns..."
kubectl patch deployment coredns -n kube-system --type=json -p='[
  {
    "op": "replace",
    "path": "/spec/template/spec/containers/0/resources",
    "value": {
      "requests": { "cpu": "10m", "memory": "32Mi" },
      "limits":   { "cpu": "100m", "memory": "128Mi" }
    }
  }
]'

# -----------------------------------------------------------------------------
# Metrics Server — uso real ~29Mi, tinha req=100m/70Mi
# -----------------------------------------------------------------------------
echo "🔧 Patching metrics-server..."
kubectl patch deployment metrics-server -n kube-system --type=json -p='[
  {
    "op": "replace",
    "path": "/spec/template/spec/containers/0/resources",
    "value": {
      "requests": { "cpu": "10m", "memory": "32Mi" },
      "limits":   { "cpu": "50m", "memory": "64Mi" }
    }
  }
]'

# -----------------------------------------------------------------------------
# Traefik — uso real ~37Mi, sem nenhum requests/limits configurados
# -----------------------------------------------------------------------------
echo "🔧 Patching traefik..."
kubectl patch deployment traefik -n kube-system --type=json -p='[
  {
    "op": "replace",
    "path": "/spec/template/spec/containers/0/resources",
    "value": {
      "requests": { "cpu": "10m", "memory": "48Mi" },
      "limits":   { "cpu": "200m", "memory": "128Mi" }
    }
  }
]'

# -----------------------------------------------------------------------------
# RabbitMQ Cluster Operator — uso real ~22Mi, tinha req=200m/500Mi
# -----------------------------------------------------------------------------
echo "🔧 Patching rabbitmq-cluster-operator..."
kubectl patch deployment rabbitmq-cluster-operator -n rabbitmq-system --type=json -p='[
  {
    "op": "replace",
    "path": "/spec/template/spec/containers/0/resources",
    "value": {
      "requests": { "cpu": "10m", "memory": "64Mi" },
      "limits":   { "cpu": "100m", "memory": "128Mi" }
    }
  }
]'

echo ""
echo "✅ Patches aplicados! Aguardando rollouts..."
kubectl rollout status deployment/coredns -n kube-system --timeout=120s
kubectl rollout status deployment/metrics-server -n kube-system --timeout=120s
kubectl rollout status deployment/traefik -n kube-system --timeout=120s
kubectl rollout status deployment/rabbitmq-cluster-operator -n rabbitmq-system --timeout=120s

echo ""
echo "📊 Uso atual após patches:"
kubectl top pods -n kube-system
echo "---"
kubectl top pods -n rabbitmq-system
