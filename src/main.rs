use rkmpi_sys::*;
use std::mem::MaybeUninit;
use std::io::Write;
fn main() {
    unsafe{get_cam_stream();}
    loop{

    }
}


pub unsafe fn init_vi_dev(dev_id: i32){
    unsafe{ 
        let mut dev_attr = VI_DEV_ATTR_S::default();
        let ret = RK_MPI_VI_GetDevAttr(dev_id, &mut dev_attr);
         if ret != 0 {
        RK_MPI_VI_SetDevAttr(dev_id, &dev_attr);
        }
        else{
            println!("DEV ALREADY SET ATTR!!!")
        }
        let ret = RK_MPI_VI_GetDevIsEnable(dev_id);
        if ret != 0 {
            let pipe_id = dev_id;
            let pipe_attr =VI_PIPE_ATTR_S::default();
            //RK_MPI_VI_CreatePipe(pipe_id as i32, &pipe_attr);
            RK_MPI_VI_EnableDev(dev_id);
            let mut pipe =VI_DEV_BIND_PIPE_S::default();
            pipe.u32Num = 1;
            pipe.PipeId[0] = pipe_id;
            RK_MPI_VI_SetDevBindPipe(dev_id, &pipe);
        }
        else{
            println!("DEV ALREADY ENABLE!!!")
        }
        }
}

pub unsafe fn init_vi_chn(channel_id:i32,width:u32,height:u32){
	unsafe{
		let mut buf_cnt = 4; // повысил с 2 до 4
		let mut chn_attr =VI_CHN_ATTR_S::default();
		chn_attr.stIspOpt.u32BufCount = buf_cnt;
		chn_attr.stIspOpt.enMemoryType = rkVI_V4L2_MEMORY_TYPE_VI_V4L2_MEMORY_TYPE_DMABUF;
		chn_attr.stSize.u32Width = width;
		chn_attr.stSize.u32Height = height;
		chn_attr.enPixelFormat = rkPIXEL_FORMAT_E_RK_FMT_YUV420SP;
		chn_attr.enCompressMode = rkCOMPRESS_MODE_E_COMPRESS_MODE_NONE;
        let mut framerate=FRAME_RATE_CTRL_S::default();
        framerate.s32DstFrameRate =25;
        framerate.s32SrcFrameRate =25;
		chn_attr.stFrameRate = framerate;
		chn_attr.u32Depth = 0;   
		chn_attr.enAllocBufType = 0; 
		chn_attr.stIspOpt.bNoUseLibV4L2 = 0;
		chn_attr.stIspOpt.aEntityName = [0; 32]; 
		chn_attr.stIspOpt.enCaptureType = 0;
		let mut ret = RK_MPI_VI_SetChnAttr(0, channel_id, &chn_attr);
		ret += RK_MPI_VI_EnableChn(0, channel_id);   
		if ret !=0{
			println!("Enable Chn RET: {}",ret);
		}   
	}
	println!("RK MPI VI CHN SUCCES INIT!!!");
}

pub unsafe fn get_raw_frame(){
	unsafe{
		let mut vi_frame_info = VIDEO_FRAME_INFO_S::default();
		loop{
			let ret = RK_MPI_VI_GetChnFrame(0, 0, &mut vi_frame_info, 1000);
			if ret == 0 {
				println!("RES WIDTH {:?}",vi_frame_info.stVFrame.u32Width);
				let adr = RK_MPI_MB_Handle2VirAddr(vi_frame_info.stVFrame.pMbBlk) as *const u8;
				if adr.is_null(){
					println!("Error! Null adress!");
				}else{
					let data_size = (vi_frame_info.stVFrame.u32Width * vi_frame_info.stVFrame.u32Height * 3 / 2) as usize;
					let data_slice = std::slice::from_raw_parts(adr,data_size);
					let filename = format!("frame_{}.nv12", vi_frame_info.stVFrame.u64PTS);
					if let Err(e) = std::fs::write(&filename,data_slice){
						println!("Failed to save data!");
					}
					else{
						println!("Save data to {}",filename);
					}
				}
				RK_MPI_VI_ReleaseChnFrame(0, 0, &vi_frame_info);
			}
			println!("RES {}",ret);
		}
	}
}

pub unsafe fn init_venc(channel_id:i32,width:u32,height:u32,rk_codec_id:rkCODEC_ID_E){
	unsafe{
		let mut venc_chn_attr = VENC_CHN_ATTR_S::default();
		let mut venc_chn_param = VENC_CHN_PARAM_S::default();

		venc_chn_attr.stVencAttr.enType = rk_codec_id;
		venc_chn_attr.stVencAttr.enPixelFormat = rkPIXEL_FORMAT_E_RK_FMT_YUV420SP;
		venc_chn_attr.stVencAttr.u32PicWidth = width;
		venc_chn_attr.stVencAttr.u32PicHeight = height;
		venc_chn_attr.stVencAttr.u32VirWidth = width;
		venc_chn_attr.stVencAttr.u32VirHeight = height;
		venc_chn_attr.stVencAttr.u32StreamBufCnt = 2;
		venc_chn_attr.stVencAttr.u32BufSize = width * height * 3 / 2;
		venc_chn_attr.stVencAttr.enMirror =  rkMIRROR_E_MIRROR_NONE;
		match rk_codec_id{
            rkCODEC_ID_E_RK_VIDEO_ID_JPEG => {
                venc_chn_attr.stVencAttr.__bindgen_anon_1.stAttrJpege.bSupportDCF = 0;
                venc_chn_attr.stVencAttr.__bindgen_anon_1.stAttrJpege.stMPFCfg.u8LargeThumbNailNum = 0;
                venc_chn_attr.stVencAttr.__bindgen_anon_1.stAttrJpege.enReceiveMode = rkVENC_PIC_RECEIVE_MODE_E_VENC_PIC_RECEIVE_SINGLE;
			},
            rkCODEC_ID_E_RK_VIDEO_ID_MJPEG => {
				venc_chn_attr.stRcAttr.enRcMode = rkVENC_RC_MODE_E_VENC_RC_MODE_MJPEGCBR;
				venc_chn_attr.stRcAttr.__bindgen_anon_1.stMjpegCbr.u32BitRate = 10 *1024;
			},
			rkCODEC_ID_E_RK_VIDEO_ID_AVC => {
				venc_chn_attr.stRcAttr.enRcMode = rkVENC_RC_MODE_E_VENC_RC_MODE_H264CBR;
				venc_chn_attr.stRcAttr.__bindgen_anon_1.stH264Cbr.u32Gop = 60;
				venc_chn_attr.stRcAttr.__bindgen_anon_1.stH264Cbr.u32SrcFrameRateNum = 25;
				venc_chn_attr.stRcAttr.__bindgen_anon_1.stH264Cbr.u32SrcFrameRateDen = 25;
				venc_chn_attr.stRcAttr.__bindgen_anon_1.stH264Cbr.fr32DstFrameRateNum = 25;
				venc_chn_attr.stRcAttr.__bindgen_anon_1.stH264Cbr.fr32DstFrameRateDen = 2;
				venc_chn_attr.stRcAttr.__bindgen_anon_1.stH264Cbr.u32BitRate = 10 * 1024;
				venc_chn_attr.stRcAttr.__bindgen_anon_1.stH264Cbr.u32StatTime = 3;
				venc_chn_attr.stVencAttr.u32Profile = 100;
			}
			rkCODEC_ID_E_RK_VIDEO_ID_HEVC => {
				venc_chn_attr.stRcAttr.enRcMode = rkVENC_RC_MODE_E_VENC_RC_MODE_H265CBR;
				venc_chn_attr.stRcAttr.__bindgen_anon_1.stH264Cbr.u32Gop = 60;
				venc_chn_attr.stRcAttr.__bindgen_anon_1.stH264Cbr.u32SrcFrameRateNum = 25;
				venc_chn_attr.stRcAttr.__bindgen_anon_1.stH264Cbr.u32SrcFrameRateDen = 1;
				venc_chn_attr.stRcAttr.__bindgen_anon_1.stH264Cbr.fr32DstFrameRateNum = 25;
				venc_chn_attr.stRcAttr.__bindgen_anon_1.stH264Cbr.fr32DstFrameRateDen = 1;
				venc_chn_attr.stRcAttr.__bindgen_anon_1.stH264Cbr.u32BitRate = 10 * 1024;
				venc_chn_attr.stRcAttr.__bindgen_anon_1.stH264Cbr.u32StatTime = 3;
				venc_chn_attr.stVencAttr.u32Profile = 100;
			}
			_ => {}
		}

		RK_MPI_VENC_CreateChn(channel_id, &venc_chn_attr);

		venc_chn_param.stFrameRate.bEnable = 0; //RK_FALSE
		venc_chn_param.stFrameRate.s32SrcFrmRateNum = 25;
		venc_chn_param.stFrameRate.s32SrcFrmRateDen = 1;
		venc_chn_param.stFrameRate.s32DstFrmRateNum = 10;
		venc_chn_param.stFrameRate.s32DstFrmRateDen = 1;

		RK_MPI_VENC_SetChnParam(channel_id, &venc_chn_param);

		let mut venc_recv_param =VENC_RECV_PIC_PARAM_S::default();
		venc_recv_param.s32RecvPicNum = -1;
		RK_MPI_VENC_StartRecvFrame(0, &venc_recv_param);
	}
}

pub unsafe fn get_venc_jpeg(){
	let mut ret: RK_S32;
	unsafe{
		ret = RK_MPI_SYS_Init();

		init_vi_dev(0);
		init_vi_chn(0,  1920, 1080);

		init_venc(0,1920,1080,rkCODEC_ID_E_RK_VIDEO_ID_JPEG);

        let mut src_chn = MPP_CHN_S::default();
        src_chn.enModId = rkMOD_ID_E_RK_ID_VI;
        src_chn.s32ChnId=0;
        src_chn.s32DevId=0;

        let mut dst_chn = MPP_CHN_S::default();
        dst_chn.enModId = rkMOD_ID_E_RK_ID_VENC;
        dst_chn.s32ChnId=0;
        dst_chn.s32DevId=0;

		ret = RK_MPI_SYS_Bind(&src_chn,&dst_chn);

		let mut jpeg_param = VENC_JPEG_PARAM_S::default();
		jpeg_param.u32Qfactor = 77;
		RK_MPI_VENC_SetJpegParam(0,&jpeg_param);


		std::thread::sleep(std::time::Duration::from_millis(30));

		let mut frame = VENC_STREAM_S::default();
		let mut pack = VENC_PACK_S::default();
		frame.pstPack = &mut pack;
										
		ret = RK_MPI_VENC_GetStream(0, &mut frame, 500);
		if ret == 0{
			let adr = RK_MPI_MB_Handle2VirAddr((*frame.pstPack).pMbBlk) as *const u8;
			let data_size = (*frame.pstPack).u32Len as usize;
			let data_slice = std::slice::from_raw_parts(adr,data_size);
			
			let filename = "frame.jpeg";
			if let Err(e) = std::fs::write(&filename,data_slice){
					println!("Failed to save data!");
				}
				else{
					println!("Save data to {}",filename);
				}

			RK_MPI_VENC_ReleaseStream(0, &mut frame);
		}

		unsafe { drop(Box::from_raw(frame.pstPack)); }
		

		ret = RK_MPI_SYS_UnBind(&src_chn,&dst_chn);
		if ret != 0{
			println!("RK MPI SYS UnBind Fail with RET: {}", ret);
		}

		RK_MPI_VI_DisableChn(0, 0);

		RK_MPI_VENC_StopRecvFrame(0);
		RK_MPI_VENC_DestroyChn(0);

		RK_MPI_VI_DisableDev(0);
		RK_MPI_SYS_Exit();
	}
}

pub unsafe fn get_cam_stream(){
	let heandle = std::thread::Builder::new().name("MJPEG STREAME".to_string()).spawn(move || {
		unsafe{
			let mut stream = std::net::TcpStream::connect("10.0.0.0:8080").unwrap();
			let mut ret;
			ret = RK_MPI_SYS_Init();

			init_vi_dev(0);
			init_vi_chn(0,  1920, 1080);
			
            init_venc(0,1920,1080,rkCODEC_ID_E_RK_VIDEO_ID_JPEG);
            
            let mut src_chn = MPP_CHN_S::default();
            src_chn.enModId = rkMOD_ID_E_RK_ID_VI;
            src_chn.s32ChnId=0;
            src_chn.s32DevId=0;

            let mut dst_chn = MPP_CHN_S::default();
            dst_chn.enModId = rkMOD_ID_E_RK_ID_VENC;
            dst_chn.s32ChnId=0;
            dst_chn.s32DevId=0;

			
			ret = RK_MPI_SYS_Bind(&src_chn,&dst_chn);

			let mut jpeg_param = VENC_JPEG_PARAM_S::default();

			jpeg_param.u32Qfactor = 77;
			RK_MPI_VENC_SetJpegParam(0,&jpeg_param);

			//rgn_overlay_procces(1920,1080);

			loop{
			    let mut frame = VENC_STREAM_S::default();
                let mut pack = VENC_PACK_S::default();
                frame.pstPack = &mut pack;
				ret = RK_MPI_VENC_GetStream(0, &mut frame, 1000);
				if ret == 0{
					let adr = RK_MPI_MB_Handle2VirAddr((*frame.pstPack).pMbBlk) as *const u8;
					let data_size = (*frame.pstPack).u32Len as usize;
					let data_slice = std::slice::from_raw_parts(adr,data_size);
					stream.write(data_slice);

					RK_MPI_VENC_ReleaseStream(0, &mut frame);
				}
				else {
					println!("FRAME RECV RET ERR = {} ",ret);
				}
				unsafe { drop(Box::from_raw(frame.pstPack)); }
				std::thread::sleep(std::time::Duration::from_millis(30));
			}
			
			ret = RK_MPI_SYS_UnBind(&src_chn,&dst_chn);
			if ret != 0{
				println!("RK MPI SYS UnBind Fail with RET: {}", ret);
			}

			RK_MPI_VI_DisableChn(0, 0);

			RK_MPI_VENC_StopRecvFrame(0);
			RK_MPI_VENC_DestroyChn(0);

			RK_MPI_VI_DisableDev(0);
			RK_MPI_SYS_Exit();

		}
	});
}

pub fn rgn_overlay_start(width:u32,height:u32){
	unsafe{
		let rgn_handle: u32 = 0;

		let mut rgn_attr  = RGN_ATTR_S::default();
		rgn_attr.enType = rkRGN_TYPE_E_OVERLAY_RGN;
		rgn_attr.unAttr.stOverlay.enPixelFmt = rkPIXEL_FORMAT_E_RK_FMT_ARGB8888;
		rgn_attr.unAttr.stOverlay.stSize.u32Width = width;
		rgn_attr.unAttr.stOverlay.stSize.u32Height = height;
		rgn_attr.unAttr.stOverlay.u32ClutNum = 0;
		rgn_attr.unAttr.stOverlay.u32CanvasNum = 2;

		let mut ret = RK_MPI_RGN_Create(rgn_handle, &rgn_attr);
		println!("RGN CREATE RET {}",ret);

		let mut mpp_chn = MPP_CHN_S::default();
		mpp_chn.enModId = rkMOD_ID_E_RK_ID_VENC;
		mpp_chn.s32DevId = 0;
		mpp_chn.s32ChnId = 0;

		let mut rgn_chn_attr = RGN_CHN_ATTR_S::default();
		rgn_chn_attr.bShow = 1;
		rgn_chn_attr.enType = rkRGN_TYPE_E_OVERLAY_RGN;
		rgn_chn_attr.unChnAttr.stOverlayChn.stPoint.s32X = 1920/2;
		rgn_chn_attr.unChnAttr.stOverlayChn.stPoint.s32Y = 1080/2;
		rgn_chn_attr.unChnAttr.stOverlayChn.u32BgAlpha = 0;
		rgn_chn_attr.unChnAttr.stOverlayChn.u32FgAlpha = 0;
		rgn_chn_attr.unChnAttr.stOverlayChn.u32Layer = 0;
		rgn_chn_attr.unChnAttr.stOverlayChn.stQpInfo.bEnable = 0;
		rgn_chn_attr.unChnAttr.stOverlayChn.stQpInfo.bForceIntra = 1;
		rgn_chn_attr.unChnAttr.stOverlayChn.stQpInfo.bAbsQp = 0;
		rgn_chn_attr.unChnAttr.stOverlayChn.stQpInfo.s32Qp = 0;
		rgn_chn_attr.unChnAttr.stOverlayChn.u32ColorLUT[0] = 0x00;
		rgn_chn_attr.unChnAttr.stOverlayChn.u32ColorLUT[1] = 0xFFFFFF;
		rgn_chn_attr.unChnAttr.stOverlayChn.stInvertColor.bInvColEn = 0;
		rgn_chn_attr.unChnAttr.stOverlayChn.stInvertColor.stInvColArea.u32Width = 16;
		rgn_chn_attr.unChnAttr.stOverlayChn.stInvertColor.stInvColArea.u32Height = 16;
		rgn_chn_attr.unChnAttr.stOverlayChn.stInvertColor.enChgMod = rkINVERT_COLOR_MODE_E_LESSTHAN_LUM_THRESH;
		rgn_chn_attr.unChnAttr.stOverlayChn.stInvertColor.u32LumThresh = 100;


		ret = RK_MPI_RGN_AttachToChn(rgn_handle, &mpp_chn, &rgn_chn_attr);

		println!("ATTACH RET {}",ret);
	}
}

pub fn rgn_overlay_procces(width:u32,height:u32){ //размеры кратны 16
	unsafe{
	
		let mut canvas_info = RGN_CANVAS_INFO_S::default();

		let mut ret = RK_MPI_RGN_GetCanvasInfo(0, &mut canvas_info);
		// println!("CANVAS INFO {:?}",canvas_info);
		// println!("logical: {}x{}, virtual: {}x{}", 
		// canvas_info.width, canvas_info.height,
		// canvas_info.size.u32_width, canvas_info.size.u32_height);
		let ptr = canvas_info.u64VirAddr as *mut u32;
		let size = (canvas_info.u32VirWidth * canvas_info.u32VirHeight) as usize;
		let slice = std::slice::from_raw_parts_mut(ptr, size);
		slice.fill(0x0000FFFF);

		// Красный	0x0000FFFF
		// Зелёный	0x0000FF00
		// Синий	0x00FF0000
		// Белый	0xFFFFFFFF
		// Чёрный	0xFF000000

		ret = RK_MPI_RGN_UpdateCanvas(0);

		// println!("CANVAS UPDATE RET {}",ret);

	}
}