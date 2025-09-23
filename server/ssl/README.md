# SSL Configuration

Este diretório contém scripts e documentação para configurar certificados SSL usando Let's Encrypt.

## 🔐 Solução Implementada

**Método**: Let's Encrypt + Certbot (standalone)  
**Vantagens**: Simples, sem AWS credentials, renovação automática  
**Certificados**: Wildcard-like para todos os subdomínios  

## 📁 Arquivos

### Scripts de Servidor (.server.sh)

- `generate-certificates.server.sh` - **Gerar certificados** (executar no servidor via SSH)
- `renew-certificates.server.sh` - **Renovar certificados** (cron no servidor)

## 🚀 Instalação Initial

### 1. Parar Nginx Temporariamente

```bash
# Local (com kubectl)
kubectl delete daemonset nginx-proxy
```

### 2. Gerar Certificados SSL

```bash
# No servidor Lightsail (via SSH)
ssh -i ~/.ssh/MyPrivateLightsail.pem ubuntu@100.27.66.245
cd /path/to/scripts
./generate-certificates.server.sh
```

### 3. Recriar Nginx com SSL

```bash
# Local (com kubectl)
kubectl apply -f nginx/nginx-deployment.yml
kubectl apply -f nginx/nginx-config.yml
```

## 🔄 Renovação Automática

### Configurar Cron no Servidor

```bash
# No servidor Lightsail
sudo crontab -e

# Adicionar linha:
0 2 * * 0 /home/ubuntu/renew-certificates.server.sh
```

## 🌐 Domínios Configurados

- ✅ **k8s.z33p.com** - APIs e landing page
- ✅ **k8s-dashboard.z33p.com** - Kubernetes Dashboard  
- ✅ **rabbitmq.z33p.com** - RabbitMQ Management

## 📋 Comandos Úteis

### Verificar Certificados

```bash
# No servidor
sudo certbot certificates
sudo openssl x509 -in /etc/letsencrypt/live/k8s.z33p.com/cert.pem -text -noout
```

### Testar Renovação

```bash
# No servidor
sudo certbot renew --dry-run
```

### Verificar Nginx

```bash
# Local
kubectl get pods -l app=nginx-proxy
kubectl logs -l app=nginx-proxy
```

## 🛡️ Firewall Requirements

### AWS Lightsail Firewall

- **Port 22 (SSH)**: Restricted IPs
- **Port 80 (HTTP)**: 0.0.0.0/0 (para Let's Encrypt)
- **Port 443 (HTTPS)**: 0.0.0.0/0 (para serviços)
- **Port 6443 (K8s)**: Restricted IPs

## ⚠️ Troubleshooting

### Erro: "Timeout during connect"

- Verificar DNS: `nslookup k8s.z33p.com`
- Verificar firewall: liberar porta 80
- Verificar nginx parado: `sudo netstat -tlnp | grep :80`

### Erro: "Certificate not found"

- Verificar caminhos em nginx-config.yml
- Verificar permissões: `ls -la /etc/letsencrypt/live/`

### Nginx não inicia

- Verificar logs: `kubectl logs -l app=nginx-proxy`
- Verificar volumes: certificados montados corretamente
- Verificar sintaxe: nginx config válida

## 🔗 Links Úteis

- [Let's Encrypt Documentation](https://letsencrypt.org/docs/)
- [Certbot User Guide](https://certbot.eff.org/instructions)
- [Nginx SSL Configuration](https://nginx.org/en/docs/http/configuring_https_servers.html)
