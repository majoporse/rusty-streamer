from opentelemetry import trace
from opentelemetry.sdk.trace import TracerProvider
from opentelemetry.sdk.trace.export import BatchSpanProcessor
from opentelemetry.exporter.otlp.proto.http.trace_exporter import OTLPSpanExporter

# -------------------------------
# 1. Configure the TracerProvider
# -------------------------------
trace.set_tracer_provider(TracerProvider())
tracer_provider = trace.get_tracer_provider()

# -------------------------------
# 2. Configure OTLP exporter to send to localhost:4318
# -------------------------------
otlp_exporter = OTLPSpanExporter(
    endpoint="http://host.docker.internal:4318/v1/traces",  # OTLP HTTP receiver
    headers={},  # optional headers
)

# Use a batch processor (recommended for production)
span_processor = BatchSpanProcessor(otlp_exporter)
tracer_provider.add_span_processor(span_processor)

# -------------------------------
# 3. Create a tracer and send a span
# -------------------------------
tracer = trace.get_tracer(__name__)

with tracer.start_as_current_span("foo"):
    print("Hello world!")  # your code that is traced

# Optional: wait a moment to ensure spans are exported
import time
time.sleep(2)
