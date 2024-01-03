# Use a imagem oficial do RabbitMQ com o plugin de gerenciamento já incluído
FROM rabbitmq:3-management

# Expõe as portas padrão do RabbitMQ e da interface de gerenciamento
EXPOSE 15672
EXPOSE 5672

# Adiciona um arquivo de configuração personalizado e scripts (se necessário)
COPY rabbitmq.conf /etc/rabbitmq/
# COPY custom_definitions.json /etc/rabbitmq/
COPY docker-entrypoint.sh /docker-entrypoint.sh

# Garanta permissões adequadas para os scripts e arquivos de configuração
RUN chmod +x /docker-entrypoint.sh
# RUN chown rabbitmq:rabbitmq /etc/rabbitmq/rabbitmq.conf /etc/rabbitmq/custom_definitions.json
RUN chown rabbitmq:rabbitmq /etc/rabbitmq/rabbitmq.conf

# Define o ponto de entrada personalizado
ENTRYPOINT ["/docker-entrypoint.sh"]

# Define o comando padrão
CMD ["rabbitmq-server"]
