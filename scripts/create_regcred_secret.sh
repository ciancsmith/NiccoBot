#!/bin/bash

##NOTE docker hub server is https://index.docker.io/v1/

# Usage function to display help if parameters are missing
usage() {
    echo "Usage: $0 --server <docker-registry-server> --username <username> --password <password> --email <email> --namespace <namespace>"
    exit 1
}

# Check for the minimum number of arguments
if [ "$#" -ne 10 ]; then
    usage
fi

# Parse command line arguments
while [[ "$#" -gt 0 ]]; do
    case $1 in
        --server) SERVER="$2"; shift ;;
        --username) USERNAME="$2"; shift ;;
        --password) PASSWORD="$2"; shift ;;
        --email) EMAIL="$2"; shift ;;
        --namespace) NAMESPACE="$2"; shift ;;
        *) echo "Unknown parameter: $1"; usage ;;
    esac
    shift
done

# Create the Kubernetes secret for Docker registry
kubectl create secret docker-registry regcred \
    --docker-server=$SERVER \
    --docker-username=$USERNAME \
    --docker-password=$PASSWORD \
    --docker-email=$EMAIL \
    --namespace=$NAMESPACE

echo "Docker registry secret created successfully."
