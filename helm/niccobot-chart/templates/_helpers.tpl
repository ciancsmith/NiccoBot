{{/*
Create a default fully qualified app name.
*/}}
{{- define "niccobot.fullname" -}}
{{- if .Values.fullnameOverride -}}
{{- .Values.fullnameOverride | trunc 63 | trimSuffix "-" -}}
{{- else -}}
{{- $name := default .Chart.Name .Values.nameOverride -}}
{{- if contains $name .Release.Name -}}
{{- .Release.Name | trunc 63 | trimSuffix "-" -}}
{{- else -}}
{{- printf "%s-%s" .Release.Name $name | trunc 63 | trimSuffix "-" -}}
{{- end -}}
{{- end -}}
{{- end -}}

{{/*
Generate labels for chart objects
*/}}
{{- define "niccobot.labels" -}}
helm.sh/chart: "{{ include "niccobot.chart" . }}"
app.kubernetes.io/name: "{{ include "niccobot.fullname" . }}"
app.kubernetes.io/instance: "{{ .Release.Name }}"
app.kubernetes.io/version: "{{ .Chart.AppVersion }}"
app.kubernetes.io/managed-by: "{{ .Release.Service }}"
{{- end -}}

{{/*
Selector labels
*/}}
{{- define "niccobot.selectorLabels" -}}
app.kubernetes.io/name: {{ include "niccobot.fullname" . }}
app.kubernetes.io/instance: {{ .Release.Name }}
{{- end -}}

{{/* Generate the chart name and version */}}
{{- define "niccobot.chart" -}}
{{ printf "%s-%s" .Chart.Name .Chart.Version | replace "+" "_" | trunc 63 | trimSuffix "-" }}
{{- end -}}
