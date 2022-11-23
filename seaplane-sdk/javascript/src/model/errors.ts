export class SeaplaneError extends Error {
  constructor(message: string) {
    super(message);

    Object.setPrototypeOf(this, SeaplaneError.prototype);
  }
}

export class HTTPError extends SeaplaneError {
  public status: number;

  constructor(status: number, message: string) {
    super(message);

    this.status = status;
    Object.setPrototypeOf(this, SeaplaneError.prototype);
  }
}
