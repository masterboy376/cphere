import { EnvelopeIcon, LockClosedIcon } from '@heroicons/react/24/outline'
import { Link } from 'react-router-dom'
import { useForm } from 'react-hook-form'
import { z } from 'zod'
import { zodResolver } from '@hookform/resolvers/zod'

import { AuthFormWrapper } from '../../wrappers/AuthFormWrapper'

const loginSchema = z.object({
  email: z.string().email(),
  password: z.string().min(8)
})

type LoginFormData = z.infer<typeof loginSchema>

export const LoginPage = () => {
  const { register, handleSubmit, formState: { errors } } = useForm<LoginFormData>({
    resolver: zodResolver(loginSchema)
  })

  const onSubmit = async (data: LoginFormData) => {
    // API call here
    console.log(data)
  }

  return (
    <AuthFormWrapper
      title="Welcome Back"
      subtitle="Please sign in to continue"
    >
      <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
        <div>
          <label className="block text-sm font-medium text-text-primary mb-2">
            Email
          </label>
          <div className="relative">
            <EnvelopeIcon className="h-5 w-5 absolute left-3 top-1/2 -translate-y-1/2 text-text-secondary" />
            <input
              {...register('email')}
              className="w-full pl-10 pr-4 py-3 bg-background rounded-lg border border-secondary/50 focus:border-primary focus:ring-2 focus:ring-primary/50"
              placeholder="email@example.com"
            />
          </div>
          {errors.email && (
            <p className="mt-1 text-sm text-red-500">{errors.email.message}</p>
          )}
        </div>

        <div>
          <label className="block text-sm font-medium text-text-primary mb-2">
            Password
          </label>
          <div className="relative">
            <LockClosedIcon className="h-5 w-5 absolute left-3 top-1/2 -translate-y-1/2 text-text-secondary" />
            <input
              type="password"
              {...register('password')}
              className="w-full pl-10 pr-4 py-3 bg-background rounded-lg border border-secondary/50 focus:border-primary focus:ring-2 focus:ring-primary/50"
              placeholder="••••••••"
            />
          </div>
          {errors.password && (
            <p className="mt-1 text-sm text-red-500">{errors.password.message}</p>
          )}
        </div>

        <div className="flex items-center justify-between">
          <Link
            to="/forgot-password"
            className="text-sm text-primary hover:text-primary-dark transition-colors"
          >
            Forgot Password?
          </Link>
        </div>

        <button
          type="submit"
          className="w-full bg-primary hover:bg-primary-dark text-white py-3 px-4 rounded-lg transition-colors"
        >
          Sign In
        </button>

        <p className="text-center text-sm text-text-secondary">
          Don't have an account?{' '}
          <Link to="/register" className="text-primary hover:text-primary-dark transition-colors">
            Register here
          </Link>
        </p>
      </form>
    </AuthFormWrapper>
  )
}