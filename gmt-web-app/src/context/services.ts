import { ApiError } from "./types";

export class main_service {
  TOKEN: string;
  BASE: string;

  constructor({
    TOKEN,
    BASE,
  }: {
    TOKEN: string | undefined;
    BASE: string | undefined;
  }) {
    if (!TOKEN) {
      throw new Error("Token not provided");
    }
    if (!BASE) {
      throw new Error("Base URL not provided");
    }
    this.TOKEN = TOKEN;
    this.BASE = BASE;
  }

  default = {
    postLogin: async (credentials: {
      email?: string;
      username?: string;
      password: string;
    }) => {
      const response = await fetch(`${this.BASE}/login`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(credentials),
      });

      if (response.ok) {
        return response.json();
      } else {
        const error = await response.json();
        throw new ApiError(
          error.message || "Invalid email, username or password"
        );
      }
    },
    postSignup: async (credentials: {
      username: string;
      email: string;
      password: string;
    }) => {
      const response = await fetch(`${this.BASE}/signup`, {
        method: "POST",
        headers: {
          "Content-Type": "application/json",
        },
        body: JSON.stringify(credentials),
      });

      if (response.ok) {
        return response.json();
      } else {
        const error = await response.json();
        throw new ApiError(
          error.message || "Invalid email, username or password"
        );
      }
    },
  };
}
