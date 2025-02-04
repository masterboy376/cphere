import { Outlet } from 'react-router-dom'
import { Footer } from '../components/common/Footer'

export const UnauthenticatedLayout = () => {
  return (
    <>
      <Outlet />
      <Footer />
    </>
  )
}