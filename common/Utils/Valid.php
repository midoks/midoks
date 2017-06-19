<?php



class Valid {

	/** 
	 * 匹配y手机号
	 * @param tel string 手机号
	 * @return bool 
	 */
	public static function tel($tel){
		$match = '/^(0|86|17951)?(13[0-9]|15[012356789]|17[678]|18[0-9]|14[57])[0-9]{8}$/';
		if(preg_match($match, $email)){
			return true;
		}
		return false;
	}

	/** 
	 * 匹配邮件
	 * @param email string 邮件地址
	 * @return bool 
	 */
	public static function email($email){
		$match = '/\w+([-+.]\w+)*@\w+([-.]\w+)*\.\w+([-.]\w+)*/';
		if(preg_match($match, $email)){
			return true;
		}
		return false;
	}

	/** 
	 * 匹配身份证
	 * @param id string 身份证
	 * @return bool 
	 */
	public static function userID($id){
		$match = '/^(^[1-9]\d{7}((0\d)|(1[0-2]))(([0|1|2]\d)|3[0-1])\d{3}$)|(^[1-9]\d{5}[1-9]\d{3}((0\d)|(1[0-2]))(([0|1|2]\d)|3[0-1])((\d{4})|\d{3}[Xx])$)$/';
		if(preg_match($match, $id)){
			return true;
		}
		return false;
	}

	/** 
	 * 匹配邮编号
	 * @param code string 邮编号
	 * @return bool 
	 */
	public static function zipCode($code){
		$match = '/^[1-9]\d{5}(?!\d)$/';
		if(preg_match($match, $code)){
			return true;
		}
		return false;
	}


	/** 
	 * 匹配http链接
	 * @param url string http链接
	 * @return bool 
	 */
	public static function url($url){
		$match = '/^https?:\/\/(([a-zA-Z0-9_-])+(\.)?)*(:\d+)?(\/((\.)?(\?)?=?&?[a-zA-Z0-9_-](\?)?)*)*$/i';
		if(preg_match($match, $code)){
			return true;
		}
		return false;
	}


}

?>