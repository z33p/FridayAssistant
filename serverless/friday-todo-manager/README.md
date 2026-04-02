# Friday Todo Manager

Serviço REST para gerenciamento de listas de tarefas e tarefas, integrado ao **Microsoft Graph API** (Microsoft To-Do). Desenvolvido em Rust com actix-web.

## Requisitos

- Rust 1.88+
- `pkg-config` e `libssl-dev` (Debian/Ubuntu: `sudo apt install pkg-config libssl-dev`)
- Acesso aos serviços `friday-secret-manager` e `friday-oauth-manager`

## Variáveis de ambiente

Crie um arquivo `.env` na raiz do projeto:

```env
SECRET_MANAGER_URL=https://k8s.z33p.com/api/friday-secret-manager
OAUTH_MANAGER_URL=https://k8s.z33p.com/api/friday-oauth-manager
```

> Em modo de desenvolvimento (debug build), valores padrão apontando para `https://k8s.z33p.com` são usados automaticamente.

## Executando localmente

```bash
cargo run
```

O servidor sobe na porta **5000**:

```
http://localhost:5000
```

## Acessando o Swagger

Com o serviço rodando, acesse a documentação interativa em:

```
http://localhost:5000/api/friday-todo-manager/swagger/
```

Em produção:

```
https://k8s.z33p.com/api/friday-todo-manager/swagger/
```

## Endpoints

### Todo Lists — `/api/friday-todo-manager/lists`

| Método | Rota | Descrição |
|--------|------|-----------|
| `GET` | `/lists` | Lista todas as listas de tarefas |
| `GET` | `/lists/{list_id}` | Retorna uma lista específica |
| `POST` | `/lists` | Cria uma nova lista |
| `PUT` | `/lists/{list_id}` | Atualiza uma lista |
| `DELETE` | `/lists/{list_id}` | Remove uma lista |

### Tasks — `/api/friday-todo-manager/lists/{list_id}/tasks`

| Método | Rota | Descrição |
|--------|------|-----------|
| `GET` | `/lists/{list_id}/tasks` | Lista todas as tarefas da lista |
| `GET` | `/lists/{list_id}/tasks/{task_id}` | Retorna uma tarefa específica |
| `POST` | `/lists/{list_id}/tasks` | Cria uma nova tarefa |
| `PATCH` | `/lists/{list_id}/tasks/{task_id}` | Atualiza uma tarefa |
| `DELETE` | `/lists/{list_id}/tasks/{task_id}` | Remove uma tarefa |

## Build e deploy

O script `deploy.sh` realiza build da imagem Docker, push para o Docker Hub e aplica os manifests Kubernetes:

```bash
./deploy.sh
```

A tag da imagem é gerada automaticamente no formato `YYYY-MM-DD.vN`.

### Kubernetes (port-forward para testes locais)

```bash
# Obtenha o nome atual do pod
kubectl get pods

# Redirecione a porta
kubectl port-forward pod/<nome-do-pod> 5000:5000
```

Depois acesse `http://localhost:5000`.

## Imagem Docker

```
docker.io/z33p/friday-todo-manager
```
