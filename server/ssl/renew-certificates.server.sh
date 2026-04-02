#!/bin/bash
# =============================================================================
# SSL CERTIFICATES RENEWAL SCRIPT
# =============================================================================
# Este script renova certificados SSL automaticamente
# Execução: NO SERVIDOR LIGHTSAIL (via cron)
# Configurar cron: 0 2 * * 0 /path/to/renew-certificates.server.sh
# =============================================================================

echo "🔄 Verificando renovação de certificados SSL..."

# Garantir que o diretório webroot existe com permissões corretas
mkdir -p /var/www/certbot
chmod 755 /var/www/certbot

# Tentar renovar certificados via webroot (nginx deve estar rodando)
sudo certbot renew --quiet --no-self-upgrade \
    --webroot --webroot-path /var/www/certbot

CERTBOT_EXIT=$?

if [ $CERTBOT_EXIT -eq 0 ]; then
    echo "✅ Verificação de renovação concluída"

    # Reiniciar nginx para carregar novos certificados (se houve renovação)
    echo "🔄 Reiniciando nginx para aplicar novos certificados..."
    kubectl rollout restart daemonset/nginx-proxy
else
    echo "❌ Erro na verificação de renovação"
    exit 1
fi