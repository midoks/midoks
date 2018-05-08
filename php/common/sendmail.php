
<?php
$mail_root_path = str_replace('\\', '/', dirname(dirname(__FILE__)));
require $mail_root_path.'/include/phpmailer/PHPMailerAutoload.php';

//不使用yepf里面的邮件发送
//使用PHPMailer
function space_phpmail_send($email_to, $email_subject, $email_message) {

	$mail = new PHPMailer();
	$mail->isSMTP();

	$mail->CharSet = "UTF-8";
	$mail->Host = "mail.demo.com";
	$mail->Port = 25;
	$mail->SMTPAuth = true;
	$mail->Username = "demo";
	$mail->Password = "demo";
	$mail->setFrom('admin@demo.com', 'demo');
	$mail->addAddress($email_to);

	$mail->Subject = $email_subject;
	$mail->msgHTML($email_message);
	$mail->AltBody = '为了查看该邮件，请切换到支持 HTML 的邮件客户端';

	if (!$mail->send()) {
	    echo "fail:" . $mail->ErrorInfo;
	} else {
	    //echo "恭喜，邮件发送成功!";
	}
	return true;
}


?>