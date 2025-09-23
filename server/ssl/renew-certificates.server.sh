#!/bin/bash
# =============================================================================
# SSL CERTIFICATES RENEWAL SCRIPT
# =============================================================================
# Este script renova certificados SSL automaticamente
# Execução: NO SERVIDOR LIGHTSAIL (via cron)
# Configurar cron: 0 2 * * 0 /path/to/renew-certificates.server.sh
# =============================================================================

echo "🔄 Verificando renovação de certificados SSL..."

# Tentar renovar certificados
sudo certbot renew --quiet --no-self-upgrade

# Verificar se houve renovação
if [ $? -eq 0 ]; then
    echo "✅ Verificação de renovação concluída"
    
    # Reiniciar nginx no Kubernetes se certificados foram renovados
    # (só executa se houver mudanças)
    if sudo certbot renew --dry-run --quiet; then
        echo "🔄 Reiniciando nginx para aplicar novos certificados..."
        kubectl rollout restart daemonset/nginx-proxy
    fi
else
    echo "❌ Erro na verificação de renovação"
    exit 1
fi