import { useState, useEffect, useRef } from 'react'
import { useNavigate, useParams, useLocation } from 'react-router-dom'
import { VideoCameraSlashIcon, XMarkIcon } from '@heroicons/react/24/outline'
import { VideoCallResponse, VideoCallEnd, WebrtcAnswer, WebrtcIceCandidate, WebrtcOffer } from '../../types/WsMessageTypes'
import wsService from '../../services/ws/WsService'
import { useAuthentication } from '../../contexts/AuthenticationContext'
import userBackendApiService, { UserDetailsPayload, UserDetailsResponse } from '../../services/user/UserBackendApiService'


export const VideoCallBoard = () => {
  const { remoteUserId } = useParams<string>();
  const { authState } = useAuthentication();
  const navigate = useNavigate();
  const location = useLocation();
  const localRef = useRef<HTMLVideoElement>(null);
  const remoteRef = useRef<HTMLVideoElement>(null);
  const pcRef = useRef<RTCPeerConnection | null>(null);
  const [remoteUserDetails, setRemoteUserDetails] = useState<UserDetailsResponse>()
  const [localStream, setLocalStream] = useState<MediaStream | null>(null)
  const [callApproved, setCallApproved] = useState<boolean>(!!location.state?.accepted)
  const [callStatus, setCallStatus] = useState<string>(location.state?.accepted ? 'connecting' : 'waiting')
  const [_error, setError] = useState<string | null>(null)

  const handleEndCall = () => {
    setCallStatus('ended')
    pcRef.current?.close();
    localStream?.getTracks().forEach(t => t.stop());
    navigate(-1);
  }

  const onEndCall = () => {
    wsService.sendMessage({
      type: 'video_call_ended',
      target_user_id: remoteUserId,
    } as VideoCallEnd)
    handleEndCall()
  }

  useEffect(() => {
    window.addEventListener('beforeunload', onEndCall);
    return () => window.removeEventListener('beforeunload', onEndCall);
  }, [remoteUserId])

  useEffect(() => {
    const videoCallAceptedListener = (message: VideoCallResponse) => {
      if (message.type === 'video_call_accepted') {
        setCallApproved(true)
        setCallStatus('connecting')
      }
    }
    const videoCallDeclinedListener = (message: VideoCallResponse) => {
      if (message.type === 'video_call_declined') {
        setCallStatus('declined')
        navigate(-1)
      }
    }
    const videoCallEndedListener = (message: VideoCallEnd) => {
      if (message.type === 'video_call_ended') {
        handleEndCall()
      }
    }
    wsService.addEventListener('video_call_accepted', videoCallAceptedListener)
    wsService.addEventListener('video_call_declined', videoCallDeclinedListener)
    wsService.addEventListener('video_call_ended', videoCallEndedListener)

    return () => {
      wsService.removeEventListener('video_call_accepted', videoCallAceptedListener)
      wsService.removeEventListener('video_call_declined', videoCallDeclinedListener)
      wsService.removeEventListener('video_call_ended', videoCallEndedListener)
    }
  }, [])

  useEffect(() => {
    const fetchUserDetails = async () => {
      if (remoteUserId) {
        try {
          const data = await userBackendApiService.userDetails({ user_id: remoteUserId } as UserDetailsPayload)
          setRemoteUserDetails(data)
        } catch (err) {
          console.error('Error fetching user details:', err)
        }
      }
    }

    fetchUserDetails()
  }, [remoteUserId])

  useEffect(() => {
    if (!callApproved) return
    let pc: RTCPeerConnection
    let stream: MediaStream
    let answered = false

    const setupCaller = async () => {
      try {
        stream = await navigator.mediaDevices.getUserMedia({ video: true, audio: true })
        setLocalStream(stream)
        if (localRef.current) localRef.current.srcObject = stream

        pc = new RTCPeerConnection()
        pcRef.current = pc

        stream.getTracks().forEach(track => pc.addTrack(track, stream))

        pc.ontrack = ev => {
          if (remoteRef.current) remoteRef.current.srcObject = ev.streams[0]
          setCallStatus('connected')
        }

        pc.onicecandidate = ev => {
          if (ev.candidate) {
            wsService.sendMessage({
              type: 'webrtc_ice_candidate',
              target_user_id: remoteUserId,
              candidate: ev.candidate
            } as WebrtcIceCandidate)
          }
        }

        const offer = await pc.createOffer()
        await pc.setLocalDescription(offer)
        wsService.sendMessage({
          type: 'webrtc_offer',
          target_user_id: remoteUserId,
          offer
        } as WebrtcOffer)
      } catch (err) {
        console.error(err)
        setError('Unable to access camera or microphone')
        setCallStatus('error')
      }
    }

    const setupCallee = async (offerDesc: RTCSessionDescriptionInit) => {
      try {
        stream = await navigator.mediaDevices.getUserMedia({ video: true, audio: true })
        setLocalStream(stream)
        if (localRef.current) localRef.current.srcObject = stream

        pc = new RTCPeerConnection()
        pcRef.current = pc
        stream.getTracks().forEach(track => pc.addTrack(track, stream))

        pc.ontrack = ev => {
          if (remoteRef.current) remoteRef.current.srcObject = ev.streams[0]
          setCallStatus('connected')
        }

        pc.onicecandidate = ev => {
          if (ev.candidate) {
            wsService.sendMessage({
              type: 'webrtc_ice_candidate',
              target_user_id: remoteUserId,
              candidate: ev.candidate
            } as WebrtcIceCandidate)
          }
        }

        await pc.setRemoteDescription(new RTCSessionDescription(offerDesc))
        const answer = await pc.createAnswer()
        await pc.setLocalDescription(answer)
        wsService.sendMessage({
          type: 'webrtc_answer',
          target_user_id: remoteUserId,
          answer
        } as WebrtcAnswer)
      } catch (err) {
        console.error(err)
        setError('Unable to access camera or microphone')
        setCallStatus('error')
      }
    }

    const handleOffer = (msg: WebrtcOffer & { from: string }) => {
      if (msg.target_user_id !== authState.userId || answered) return
      answered = true
      setCallApproved(true)
      setCallStatus('connecting')
      setupCallee(msg.offer)
    }

    wsService.addEventListener('webrtc_offer', handleOffer)

    // Only caller triggers immediately
    if (location.state?.accepted === false) {
      setupCaller()
    }

    return () => {
      wsService.removeEventListener('webrtc_offer', handleOffer)
      pcRef.current?.close()
      stream?.getTracks().forEach(t => t.stop())
    }
  }, [callApproved, remoteUserId, location.state])

  // Listen for answer & ICE after PC exists
  useEffect(() => {
    const handleAnswer = (msg: WebrtcAnswer & { from: string }) => {
      if (msg.target_user_id !== authState.userId) return
      pcRef.current?.setRemoteDescription(new RTCSessionDescription(msg.answer))
    }
    const handleIce = (msg: WebrtcIceCandidate & { from: string }) => {
      if (msg.target_user_id !== authState.userId) return
      pcRef.current?.addIceCandidate(new RTCIceCandidate(msg.candidate))
    }

    wsService.addEventListener('webrtc_answer', handleAnswer)
    wsService.addEventListener('webrtc_ice_candidate', handleIce)

    return () => {
      wsService.removeEventListener('webrtc_answer', handleAnswer)
      wsService.removeEventListener('webrtc_ice_candidate', handleIce)
    }
  }, [remoteUserId])

  return (
    <div className="h-screen w-full bg-background flex flex-col">
      {/* Video Containers */}
      <div className="flex-1 grid grid-cols-1 md:grid-cols-2 gap-4 p-4">
        {/* Local Video */}
        <div className="relative bg-background-paper rounded-lg overflow-hidden">
          <video
            ref={localRef}
            autoPlay
            muted
            className="h-full w-full object-cover"
          />
          <div className="absolute bottom-2 left-2 text-sm text-white bg-black/50 px-2 py-1 rounded">
            You
          </div>
        </div>

        {/* Remote Video */}
        <div className="relative bg-background-paper rounded-lg overflow-hidden">
          {remoteRef ? (
            <video
              ref={remoteRef}
              autoPlay
              className="h-full w-full object-cover"
              onCanPlay={() => remoteRef.current?.play()}
            />
          ) : (
            <div className="h-full w-full flex items-center justify-center">
              <div className="text-center">
                <VideoCameraSlashIcon className="h-12 w-12 text-gray-500 mx-auto mb-2" />
                <p className="text-gray-400">
                  {callStatus === 'calling' ? 'Calling...' : 'Waiting for answer'}
                </p>
              </div>
            </div>
          )}
          {remoteRef && (
            <div className="absolute bottom-2 left-2 text-sm text-white bg-black/50 px-2 py-1 rounded">
              {remoteUserDetails ? remoteUserDetails.username : 'Loading...'}
            </div>
          )}
        </div>
      </div>

      {/* Controls */}
      <div className="p-4 flex justify-center bg-background-paper">
        <button
          onClick={onEndCall}
          type="button"
          className="flex items-center gap-2 px-6 py-3 bg-red-600 hover:bg-red-700 text-white rounded-full transition-colors"
        >
          <XMarkIcon className="h-6 w-6" />
          End Call
        </button>
      </div>
    </div>
  )
}