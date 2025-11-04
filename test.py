from opentelemetry import trace
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor
from opentelemetry.exporter.otlp.proto.http.trace_exporter import OTLPSpanExporter
from opentelemetry.sdk.resources import Resource

import time

# -------------------------------
# 1. Configure the TracerProvider with service name
# -------------------------------
resource = Resource.create({
    "service.name": "my-python-service",  # service name
    "service.version": "1.0.0",           # optional version tag
    "deployment.environment": "dev"       # optional environment tag
})

trace.set_tracer_provider(TracerProvider(resource=resource))
tracer_provider = trace.get_tracer_provider()

# -------------------------------
# 2. Configure OTLP exporter to send to localhost:4318
# -------------------------------
otlp_exporter = OTLPSpanExporter(
    endpoint="http://host.docker.internal:4318/v1/traces",
    headers={},  # optional headers
)

# Use a batch processor (recommended for production)
span_processor = BatchSpanProcessor(otlp_exporter)
tracer_provider.add_span_processor(span_processor)

# -------------------------------
# 3. Create a tracer and send a span with a custom tag
# -------------------------------
tracer = trace.get_tracer(__name__)

with tracer.start_as_current_span("foo") as span:
    # Add a custom tag (span attribute)
    span.set_attribute("custom.tag", "example-tag-value")
    span.set_attribute("user.id", 12345)  # example of another tag
    print("Hello world!")

# Optional: wait a moment to ensure spans are exported
time.sleep(2)
