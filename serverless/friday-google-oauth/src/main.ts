import { OAuth2Client } from 'google-auth-library';
import dotenv from 'dotenv';
import { APIGatewayProxyHandler } from 'aws-lambda';
import { LambdaResponse } from './contracts/LambdaResponse';
import { GetAccessTokenRequest, RequestData } from './contracts/RequestData';

const SCOPES: string[] = ['https://www.googleapis.com/auth/gmail.send'];

async function getAccessToken(client: OAuth2Client, data: GetAccessTokenRequest): Promise<LambdaResponse> {
  const url = data.url ?? process.env.OAUTH_URL_CODE;
  const code = extractCodeFromUrl(url);
  const { tokens } = await client.getToken(code);

  return {
    statusCode: 200,
    data: {
      tokens
    },
  };
}

async function generateOAuthUrl(client: OAuth2Client, _: any): Promise<LambdaResponse> {
  const url = client.generateAuthUrl({
    access_type: 'offline',
    scope: SCOPES,
  });

  return {
    statusCode: 200,
    data: {
      url
    },
  };
}

function extractCodeFromUrl(url: string): string | null {
  const searchParams = new URL(url).searchParams;
  const code = searchParams.get('code');
  return code;
}

async function main(request_data: RequestData): Promise<LambdaResponse> {
  dotenv.config();
  const redirectUri = "http://localhost/";
  const client = new OAuth2Client(process.env.OAUTH_CLIENT_ID, process.env.OAUTH_CLIENT_SECRET, redirectUri);
  const lambdaActions: Record<string, (client: OAuth2Client, data: any) => Promise<LambdaResponse>> = {
    "GENERATE_OAUTH_URL": generateOAuthUrl,
    "GET_ACCESS_TOKEN": getAccessToken
  };

  const response = await lambdaActions[request_data.action](client, request_data.payload);
  return response;
}

export const handler: APIGatewayProxyHandler = async (event) => {
  const request_data: RequestData = handleEventBodyAsRequestData(event.body);
  const response = await main(request_data);

  return {
    statusCode: response.statusCode,
    body: JSON.stringify(response.data)
  };
};

function handleEventBodyAsRequestData(event_body: any): RequestData {
  return typeof event_body === 'string' ? JSON.parse(event_body) : event_body;
}

