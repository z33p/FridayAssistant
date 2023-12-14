export interface RequestData {
    action: string;
    payload: GetAccessTokenRequest | null
}

export interface GetAccessTokenRequest {
    url?: string
}
