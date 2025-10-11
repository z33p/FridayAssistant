#!/bin/bash

# Script para limpeza automática de ReplicaSets antigos
# Autor: Friday Assistant
# Data: $(date +%Y-%m-%d)

set -e

echo "🧹 Iniciando limpeza de ReplicaSets antigos..."
echo "Data/Hora: $(date)"
echo "----------------------------------------"

# Função para logging
log() {
    echo "[$(date '+%Y-%m-%d %H:%M:%S')] $1"
}

# Contador de ReplicaSets removidos
REMOVED_COUNT=0

# Limpeza de ReplicaSets com DESIRED=0 (versões antigas não utilizadas)
log "Buscando ReplicaSets antigos com DESIRED=0..."

# Buscar todos os ReplicaSets com replicas=0
kubectl get replicasets --all-namespaces -o json | jq -r '
    .items[] | 
    select(.spec.replicas == 0) | 
    "\(.metadata.namespace) \(.metadata.name) \(.metadata.creationTimestamp)"
' | while read namespace name created; do
    
    # Calcular idade do ReplicaSet (em dias)
    created_timestamp=$(date -d "$created" +%s)
    current_timestamp=$(date +%s)
    age_days=$(( (current_timestamp - created_timestamp) / 86400 ))
    
    # Remover apenas ReplicaSets com mais de 1 dia (segurança)
    if [ $age_days -gt 1 ]; then
        log "Removendo ReplicaSet antigo: $namespace/$name (idade: ${age_days} dias)"
        kubectl delete replicaset "$name" -n "$namespace" --ignore-not-found=true
        REMOVED_COUNT=$((REMOVED_COUNT + 1))
    else
        log "Mantendo ReplicaSet recente: $namespace/$name (idade: ${age_days} dias)"
    fi
done

log "Limpeza concluída. ReplicaSets removidos: $REMOVED_COUNT"

# Estatísticas finais
TOTAL_RS=$(kubectl get replicasets --all-namespaces --no-headers | wc -l)
ACTIVE_RS=$(kubectl get replicasets --all-namespaces -o json | jq '[.items[] | select(.spec.replicas > 0)] | length')

echo "----------------------------------------"
echo "📊 Estatísticas finais:"
echo "   Total de ReplicaSets: $TOTAL_RS"
echo "   ReplicaSets ativos: $ACTIVE_RS"
echo "   ReplicaSets antigos removidos: $REMOVED_COUNT"
echo "🎉 Limpeza concluída com sucesso!"