import { useForm } from 'react-hook-form'
import { zodResolver } from '@hookform/resolvers/zod'
import { z } from 'zod'
import { useParams } from 'react-router-dom'
import { AuthFormWrapper } from '../../wrappers/AuthFormWrapper'
import { LockClosedIcon } from '@heroicons/react/24/outline'

const resetPasswordSchema = z.object({
  password: z.string().min(8),
  confirmPassword: z.string()
}).refine(data => data.password === data.confirmPassword, {
  message: "Passwords don't match",
  path: ["confirmPassword"]
})

type ResetPasswordData = z.infer<typeof resetPasswordSchema>        

export const ResetPasswordPage = () => {
  const { token } = useParams()
  const { register, handleSubmit, formState: { errors } } = useForm<ResetPasswordData>({
    resolver: zodResolver(resetPasswordSchema)
  })

  const onSubmit = async (data: ResetPasswordData) => {
    // API call with token
    console.log({ ...data, token })
  }

  return (
    <AuthFormWrapper
      title="Set New Password"
      subtitle="Enter your new password below"
    >
      <form onSubmit={handleSubmit(onSubmit)} className="space-y-6">
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

        <div>
          <label className="block text-sm font-medium text-text-primary mb-2">
            Confirm Password
          </label>
          <div className="relative">
            <LockClosedIcon className="h-5 w-5 absolute left-3 top-1/2 -translate-y-1/2 text-text-secondary" />
            <input
              type="password"
              {...register('confirmPassword')}
              className="w-full pl-10 pr-4 py-3 bg-background rounded-lg border border-secondary/50 focus:border-primary focus:ring-2 focus:ring-primary/50"
              placeholder="••••••••"
            />
          </div>
          {errors.confirmPassword && (
            <p className="mt-1 text-sm text-red-500">{errors.confirmPassword.message}</p>
          )}
        </div>
        
        <button
          type="submit"
          className="w-full bg-primary hover:bg-primary-dark text-white py-3 px-4 rounded-lg transition-colors"
        >
          Reset Password
        </button>
      </form>
    </AuthFormWrapper>
  )
}