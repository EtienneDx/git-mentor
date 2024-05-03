export class ApiError extends Error {
  constructor(public message: string) {
    super(message);
    this.name = "ApiError";
  }
}
