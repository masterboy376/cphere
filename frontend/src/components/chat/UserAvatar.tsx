interface UserAvatarProps {
    username: string
    size?: 'sm' | 'md' | 'lg'
  }
  
  export const UserAvatar = ({ username, size = 'md' }: UserAvatarProps) => {
    const sizeClasses = {
      sm: 'h-8 w-8 text-sm',
      md: 'h-10 w-10 text-base',
      lg: 'h-12 w-12 text-lg'
    }
  
    return (
      <div
        className={`${sizeClasses[size]} bg-primary rounded-full flex items-center justify-center text-white font-medium`}
      >
        {username[0].toUpperCase()}
      </div>
    )
  }