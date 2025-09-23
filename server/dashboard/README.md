# Kubernetes Dashboard

Este diretório contém scripts e configurações para instalar e acessar o Kubernetes Dashboard.

## 📁 Arquivos

### Scripts de Execução

- `dashboard.sh` - **Instalação completa** do Dashboard (executar localmente)
- `token-dashboard.sh` - **Gerar token** de acesso (executar localmente)  
- `port-forward_dashboard.local.sh` - **Acesso local** via port-forward (executar localmente)

### Configurações YAML

- `service-account.yaml` - Service Account para acesso administrativo
- `cluster-role-binding.yml` - Permissões de cluster admin
- `secret.yml` - Secret para token de autenticação

## 🚀 Instalação

### 1. Instalar Dashboard

```bash
./dashboard.sh
```

### 2. Gerar Token de Acesso

```bash
./token-dashboard.sh
```

## 🌐 Formas de Acesso

### Via Subdomínio (Produção)

- **URL**: <https://k8s-dashboard.z33p.com>
- **Método**: Token de acesso

### Via Port-Forward (Desenvolvimento)

```bash
./port-forward_dashboard.local.sh
```

- **URL**: <https://localhost:8443>
- **Método**: Token de acesso

## 🔐 Autenticação

O Dashboard usa token de acesso JWT. Para obter:

1. Execute: `./token-dashboard.sh`
2. Copie o token exibido
3. No Dashboard, escolha "Token" e cole o token
4. **Nota**: Token expira em 1 hora

## 📋 Comandos Úteis

```bash
# Ver pods do Dashboard
kubectl get pods -n kubernetes-dashboard

# Ver logs do Dashboard
kubectl logs -n kubernetes-dashboard deployment/kubernetes-dashboard-web

# Restart Dashboard
kubectl rollout restart deployment/kubernetes-dashboard-web -n kubernetes-dashboard

# Verificar serviços
kubectl get svc -n kubernetes-dashboard
```

## ⚠️ Pré-requisitos

- Cluster K3s funcionando
- kubectl configurado
- Helm instalado
- Para subdomínio: DNS configurado para `k8s-dashboard.z33p.com`
