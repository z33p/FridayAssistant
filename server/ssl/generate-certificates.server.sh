#!/bin/bash
# =============================================================================
# SSL CERTIFICATES INSTALLATION SCRIPT
# =============================================================================
# Este script configura certificados SSL usando Let's Encrypt + Certbot
# Execução: NO SERVIDOR LIGHTSAIL (via SSH)
# Pré-requisitos: Nginx parado temporariamente + DNS configurado
# =============================================================================

echo "🔐 Configurando certificados SSL..."

# Verificar se certbot está instalado
if ! command -v certbot &> /dev/null; then
    echo "📦 Instalando certbot..."
    sudo apt update
    sudo apt install -y certbot
fi

# Verificar se nginx está parado (necessário para standalone)
echo "⚠️  IMPORTANTE: Certifique-se que o nginx está parado!"
echo "   No kubectl: kubectl delete daemonset nginx-proxy"
echo ""
read -p "Nginx está parado? (y/n): " -n 1 -r
echo
if [[ ! $REPLY =~ ^[Yy]$ ]]; then
    echo "❌ Pare o nginx primeiro e execute novamente!"
    exit 1
fi

# Gerar certificados para todos os subdomínios
echo "🌐 Gerando certificados SSL para subdomínios..."
sudo certbot certonly --standalone \
  -d k8s.z33p.com \
  -d k8s-dashboard.z33p.com \
  -d rabbitmq.z33p.com \
  --email raphael00fellipe@gmail.com \
  --agree-tos \
  --no-eff-email \
  --non-interactive

if [ $? -eq 0 ]; then
    echo "✅ Certificados SSL gerados com sucesso!"
    echo ""
    echo "📋 Certificados salvos em:"
    echo "   /etc/letsencrypt/live/k8s.z33p.com/"
    echo ""
    echo "🔄 Próximos passos:"
    echo "1. Recriar nginx: kubectl apply -f nginx/nginx-deployment.yml"
    echo "2. Aplicar config SSL: kubectl apply -f nginx/nginx-config.yml"
    echo "3. Testar: https://k8s-dashboard.z33p.com"
else
    echo "❌ Erro ao gerar certificados!"
    echo "Verifique:"
    echo "- DNS está apontando para o servidor"
    echo "- Firewall libera porta 80"
    echo "- Nginx está realmente parado"
    exit 1
fi