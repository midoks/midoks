<?php
class Ase {
	public $_cryptoKey = "123";

	public function decryptAES($encryptedText) {
		$cipher     = 'AES-128-CBC';
		$ciphertext = base64_decode($encryptedText);
		$ivSize     = openssl_cipher_iv_length($cipher);
		$iv         = substr($ciphertext, 0, $ivSize);
		$ciphertext = substr($ciphertext, $ivSize);

		$decrypted = openssl_decrypt($ciphertext, $cipher, $this->_cryptoKey, 1, $iv);
		if ($decrypted === false) {
			return false;
		}

		return $decrypted;
	}

	public function encryptAES($ciphertext) {
		$cipher = 'AES-128-CBC';
		$ivSize = openssl_cipher_iv_length($cipher);
		$iv     = openssl_random_pseudo_bytes($ivSize);

		$ciphertext = $iv . $ciphertext;
		$decrypted  = openssl_encrypt($ciphertext, $cipher, $this->_cryptoKey, 1, $iv);

		return base64_encode($decrypted);
	}
}