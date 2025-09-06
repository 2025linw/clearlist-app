export function isSuccess(status: number): boolean {
  return 200 <= status && status < 300;
}

export function isUserErr(status: number): boolean {
  return 400 <= status && status < 500;
}

export function isServerErr(status: number): boolean {
  return 500 <= status && status < 600;
}
