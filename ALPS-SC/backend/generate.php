<?php
require "config.php";
$url = $BACKEND_URL . "/token/generate";
$data = [
  "id" => $TOKEN_PASSWORD,
];
$jsonData = json_encode($data);
$ch = curl_init($url);
curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
curl_setopt($ch, CURLOPT_POST, true);
curl_setopt($ch, CURLOPT_POSTFIELDS, $jsonData);
curl_setopt($ch, CURLOPT_HTTPHEADER, [
  "Content-Type: application/json",
  "Content-Length: " . strlen($jsonData),
]);
$response = curl_exec($ch);
if ($response === false) {
  http_response_code(500);
  echo json_encode(["id" => ""]);
  curl_close($ch);
  exit;
}
$httpCode = curl_getinfo($ch, CURLINFO_HTTP_CODE);
curl_close($ch);
header('Content-Type: application/json; charset=utf-8');
http_response_code($httpCode);
echo $response;
