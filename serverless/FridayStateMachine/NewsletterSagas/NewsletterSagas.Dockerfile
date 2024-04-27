# Use the dotnet8 image as the base image
FROM mcr.microsoft.com/dotnet/sdk:8.0 AS build

# Set the working directory inside the container
WORKDIR /app

# Copy the project files to the container
COPY Libs/ ./Libs
COPY NewsletterSagas/ ./NewsletterSagas

# Restore the NuGet packages
RUN dotnet restore NewsletterSagas/NewsletterSagas.csproj --verbosity detailed

# Build the project
RUN dotnet build NewsletterSagas/NewsletterSagas.csproj -c Release -o /app/build

# Publish the project
RUN dotnet publish NewsletterSagas/NewsletterSagas.csproj -c Release -o /app/publish

# Use the dotnet8 image as the base image
FROM mcr.microsoft.com/dotnet/aspnet:8.0 AS runtime

# Set the working directory inside the container
WORKDIR /app

# Copy the published files from the build stage to the runtime stage
COPY --from=build /app/publish .

# Set the entry point for the container
ENTRYPOINT ["dotnet", "NewsletterSagas.dll"]
