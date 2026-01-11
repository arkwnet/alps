<?php
require "config.php";
if ($_SERVER["REQUEST_METHOD"] === "OPTIONS") {
  http_response_code(204);
  exit;
}
header("Content-Type: application/json; charset=utf-8");
$json = file_get_contents("php://input");
if ($json === false || $json === "") {
  echo json_encode(["status" => 100]);
  exit;
}
$data = json_decode($json, true);
$verifyData = [
  "id" => $data["token"],
];
$verifyJson = json_encode($verifyData);
$url = $BACKEND_URL . "/token/verify";
$ch = curl_init($url);
curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
curl_setopt($ch, CURLOPT_POST, true);
curl_setopt($ch, CURLOPT_POSTFIELDS, $verifyJson);
curl_setopt($ch, CURLOPT_HTTPHEADER, [
  "Content-Type: application/json",
  "Content-Length: " . strlen($verifyJson),
]);
$response = curl_exec($ch);
curl_close($ch);
if ($response === false) {
  echo json_encode(["status" => 200]);
  exit;
}
$result = json_decode($response, true);
if ($result === null) {
  echo json_encode(["status" => 300]);
  exit;
}
if (isset($result["status"]) && $result["status"] === true) {
  if (isset($data["token"])) {
    unset($data["token"]);
  }
  $json2 = json_encode($data);
  $url = $BACKEND_URL . "/record";
  $ch = curl_init($url);
  curl_setopt($ch, CURLOPT_POST, true);
  curl_setopt($ch, CURLOPT_HTTPHEADER, [
    "Content-Type: application/json",
    "Content-Length: " . strlen($json),
  ]);
  curl_setopt($ch, CURLOPT_POSTFIELDS, $json);
  curl_setopt($ch, CURLOPT_RETURNTRANSFER, true);
  $response = curl_exec($ch);
  $statusCode = curl_getinfo($ch, CURLINFO_HTTP_CODE);
  curl_close($ch);
  if ($response === false) {
    echo json_encode(["status" => 500]);
    exit;
  }
  echo json_encode(["status" => 0]);
} else {
  echo json_encode(["status" => 400]);
}
