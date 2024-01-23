import axios, { AxiosError } from "axios";
import { Error, ErrorCode, LoginRequest, LoginResponse } from "@/proto";

const api_root = process.env.REACT_APP_API_ROOT || 'http://localhost:3000';
const defaultError: Error = { msg: "An unknown error occurred", code: ErrorCode.UNKNOWN };

type ApiResponse<T> = {
  success: true;
  data: T;
} | {
  success: false;
  error: Error;
};

function success<T>(data: T): ApiResponse<T> {
  return { success: true, data };
}

function failure<T>(error?: Error): ApiResponse<T> {
  return { success: false, error: error ?? defaultError };
}

// eslint-disable-next-line @typescript-eslint/no-unused-vars
async function get<T>(url: string): Promise<ApiResponse<T>> {
  try {
    const response = await axios.get<T>(url);
    return success(response.data);
  }
  catch (error) {
    const e = error as AxiosError<Error>;
    return failure(e.response?.data);
  }
}

async function post<T, U>(url: string, request: T): Promise<ApiResponse<U>> {
  try {
    const response = await axios.post<U>(url, request);
    return success(response.data);
  }
  catch (error) {
    const e = error as AxiosError<Error>;
    return failure(e.response?.data);
  }
}

export async function login(request: LoginRequest): Promise<ApiResponse<LoginResponse>> {
  return await post<LoginRequest, LoginResponse>(`${api_root}/login`, request);
}
