export function isInformational(status: number): boolean {
  return status >= 100 && status < 200;
}

export function isSuccess(status: number): boolean {
  return status >= 200 && status < 300;
}

export function isRedirect(status: number): boolean {
  return status >= 300 && status < 400;
}

export function isClientError(status: number): boolean {
  return status >= 400 && status < 500;
}

export function isServerError(status: number): boolean {
  return status >= 500 && status < 600;
}

export function nonAPIStatusCodes(status: number): boolean {
  return isInformational(status) || isRedirect(status);
}
