type BaseResponseSchema = {
  status: 'ok' | 'success' | 'error';
};

export type HealthcheckResponseSchema = BaseResponseSchema & {
  version?: string;

  message?: string;
}

export type LoginResponseSchema = BaseResponseSchema & {
  data: {
    userId: string;
    email: string;

    accessJwt: string;
    refreshJwt: string;
  };
};

export type RefreshResponseSchema = BaseResponseSchema & {
  data: { accessJwt: string; refreshJwt: string };
};
