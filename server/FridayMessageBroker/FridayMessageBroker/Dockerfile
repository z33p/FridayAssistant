FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build

COPY . /build/

RUN dotnet restore ./build/FridayMessageBroker/FridayMessageBroker.csproj

COPY ./ ./build/

WORKDIR /build/
RUN dotnet publish ./FridayMessageBroker/FridayMessageBroker.csproj -c RELEASE -o out /p:Version=1.0.0

# Use a imagem do ASP.NET Core Runtime
FROM mcr.microsoft.com/dotnet/aspnet:8.0
WORKDIR /app

COPY --from=build /build/out .

CMD ["dotnet", "FridayMessageBroker.dll"]
