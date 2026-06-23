export default {
  registration: {
    title: 'ユーザー登録',
    form: {
      fields: {
        email: {
          label: 'メールアドレス',
          validations: {
            required: 'メールアドレスを入力してください。',
            email: '有効なメールアドレスを入力してください。',
          },
        },
        password: {
          label: 'パスワード',
          validations: {
            required: 'パスワードを入力してください。',
          },
        },
        confirmPassword: {
          label: 'パスワード確認',
          validations: {
            required: 'パスワードを確認してください。',
            match: 'パスワードが一致しません。',
          },
        },
      },
      actions: {
        submit: '登録する',
      },
    },
    links: {
      login: 'すでにアカウントをお持ちですか？こちらからログイン。',
    },
    feedback: {
      success: {
        registration: '登録が完了しました！',
      },
    },
  },
  login: {
    title: 'ログイン',
    form: {
      fields: {
        email: {
          label: 'メールアドレス',
          validations: {
            required: 'メールアドレスを入力してください。',
            email: '有効なメールアドレスを入力してください。',
          },
        },
        password: {
          label: 'パスワード',
          validations: {
            required: 'パスワードを入力してください。',
          },
        },
      },
      actions: {
        submit: 'ログインする',
      },
    },
    links: {
      register: 'アカウントをお持ちでないですか？こちらから登録。',
    },
    feedback: {
      success: {
        login: 'ようこそ！',
      },
    },
  },
  monastery: {
    title: '修道院',
    monks: '修道士',
  },
}
