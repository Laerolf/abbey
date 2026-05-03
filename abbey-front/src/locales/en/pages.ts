export default {
  userRegistration: {
    title: 'User Registration',
    form: {
      fields: {
        email: {
          label: 'Email',
          validations: {
            required: 'An email is required.',
            email: 'Please enter a valid email address.',
          },
        },
        password: {
          label: 'Password',
          validations: {
            required: 'A password is required.',
          },
        },
        confirmPassword: {
          label: 'Password confirmation',
          validations: {
            required: 'Please confirm your password.',
            match: 'Please confirm your password correctly.',
          },
        },
      },
      actions: {
        submit: 'Register',
      },
    },
    links: {
      login: 'Already have an account? Login here.',
    },
    feedback: {
      success: {
        registration: 'Welcome aboard!',
      },
    },
  },
  userLogin: {
    title: 'Login',
    form: {
      fields: {
        email: {
          label: 'Email',
          validations: {
            required: 'An email is required.',
            email: 'Please enter a valid email address.',
          },
        },
        password: {
          label: 'Password',
          validations: {
            required: 'A password is required.',
          },
        },
      },
      actions: {
        submit: 'Login',
      },
    },
    links: {
      register: "Don't have an account? Register here.",
    },
    feedback: {
      success: {
        login: 'Welcome!',
      },
    },
  },
  gameMonastery: {
    title: 'Monastery',
    monks: 'Monks',
  },
}
