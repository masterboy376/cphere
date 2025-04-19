import { EnvelopeIcon, LockClosedIcon } from '@heroicons/react/24/outline'
import { Link, useNavigate } from 'react-router-dom'
import { useForm } from 'react-hook-form'
import { useState } from 'react'
import { z } from 'zod'
import { zodResolver } from '@hookform/resolvers/zod'
import { AuthFormWrapper } from '../../wrappers/AuthFormWrapper'
import authBackendApiService, { AuthLoginRequest } from '../../services/auth/AuthBackendApiService.ts'
import { useAuthentication } from '../../contexts/AuthenticationContext.tsx'

const loginSchema = z.object({
  username: z.string().min(2, "Username must be at least 2 characters").regex(/^\S*$/, "Username should not contain spaces"),
  password: z.string().min(8, "Password must be at least 8 characters"),
})

type LoginFormData = z.infer<typeof loginSchema>

export const LoginPage = () => {
  const { register, handleSubmit, formState: { errors } } = useForm<LoginFormData>({
    resolver: zodResolver(loginSchema)
  })

  const [loading, setLoading] = useState(false)
  const navigate = useNavigate()
  const [formResponse, setFormResponse] = useState<{ status: string, message: string } | null>(null)
  const { setAuthState } = useAuthentication()

  const onLogin = async (formData: LoginFormData) => {
    setLoading(true)
    setFormResponse(null)
    try {
      const payload: AuthLoginRequest = {
        username: formData.username,
        password: formData.password
      }
      // Call the login API; ensure your authBackendApiService.login is correctly implemented
      const data = await authBackendApiService.login(payload)
      // wsService.connect()
      setAuthState({ userId: data.user_id, username: data.username })
      setFormResponse({
        "status": "success",
        "message": "Authentication successful!"
      })
      navigate('/chats')
    } catch (err: any) {
      setFormResponse({
        "status": "error",
        "message": "Authentication failed!"
      })
    } finally {
      setLoading(false)
    }
  }

  return (
    <AuthFormWrapper
      title="Welcome Back"
      subtitle="Please sign in to continue"
    >
      <form onSubmit={handleSubmit(onLogin)} className="space-y-6">
        <div>
          <label className="block text-sm font-medium text-text-primary mb-2">
            Username
          </label>
          <div className="relative">
            <EnvelopeIcon className="h-5 w-5 absolute left-3 top-1/2 -translate-y-1/2 text-text-secondary" />
            <input
              {...register('username')}
              className="w-full pl-10 pr-4 py-3 bg-background rounded-lg border border-secondary/50 focus:border-primary focus:ring-2 focus:ring-primary/50"
              placeholder="example-user"
            />
          </div>
          {errors.username && (
            <p className="mt-1 text-sm text-red-500">{errors.username.message}</p>
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
          className="w-full bg-primary hover:bg-primary-dark text-white py-3 px-4 rounded-lg transition-colors disabled:bg-gray-400"
          disabled={loading}
        >
          {loading ? "Signing In..." : "Sign In"}
        </button>
        {formResponse && (
          <p className={`mt-1 text-center text-sm ${formResponse.status === "success" ? "text-green-500" : "text-red-500"}`}>{formResponse.message}</p>
        )}

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
